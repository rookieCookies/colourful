#[cfg(not(no_std))]
use std::{io::IsTerminal, sync::atomic::{AtomicU8, Ordering}};


#[cfg(not(no_std))]
static METADATA : AtomicU8 = AtomicU8::new(u8::MAX);


pub fn flush_metadata() {
    METADATA.store(u8::MAX, Ordering::Relaxed);
    init_metadata();
}


#[cfg(not(no_std))]
fn init_metadata() -> (bool, bool, bool) {
    let mut val = METADATA.load(std::sync::atomic::Ordering::Relaxed);
    if val == u8::MAX {
        let mut compute = 0b00000000;
        if std::env::var("NO_COLOR").is_ok() {
            compute += 2u8.pow(1);
        }

        
        if std::env::var("FORCE_COLOR").is_ok() {
            compute += 2u8.pow(2);
        }

        
        if std::io::stdin().is_terminal() {
            compute += 2u8.pow(3);
        }

        val = compute;
        METADATA.store(compute, std::sync::atomic::Ordering::Relaxed);
    }

    (
        (val & 2u8.pow(1) == 2u8.pow(1)),
        (val & 2u8.pow(2) == 2u8.pow(2)),
        (val & 2u8.pow(3) == 2u8.pow(3)),
    )
}



#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
}


#[derive(Clone, Copy, PartialEq)]
pub struct ColouredString<T> {
    string: T,

    fg_colour: Option<Colour>,
    bg_colour: Option<Colour>,
    is_bold: bool,
    is_dim: bool,
    is_italic: bool,
    is_underline: bool,
    is_blinking: bool,
    is_inverse: bool,
    is_hidden: bool,
    is_strikethrough: bool,
}


impl<T> ColouredString<T> {
    pub fn colour(mut self, colour: Colour) -> Self {
        self.fg_colour = Some(colour);
        self
    }

    
    pub fn bg_colour(mut self, colour: Colour) -> Self {
        self.bg_colour = Some(colour);
        self
    }


    pub fn bold(mut self) -> Self {
        self.is_bold = true;
        self
    }


    pub fn dim(mut self) -> Self {
        self.is_dim = true;
        self
    }


    pub fn italic(mut self) -> Self {
        self.is_italic = true;
        self
    }


    pub fn underline(mut self) -> Self {
        self.is_underline = true;
        self
    }


    pub fn blinking(mut self) -> Self {
        self.is_blinking = true;
        self
    }


    pub fn inverse(mut self) -> Self {
        self.is_inverse = true;
        self
    }


    pub fn hidden(mut self) -> Self {
        self.is_hidden = true;
        self
    }


    pub fn strikethrough(mut self) -> Self {
        self.is_strikethrough = true;
        self
    }


    fn write_ansi(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\u{001b}[")?;
        let mut has_updated = false;

        if let Some(Colour { r, g, b }) = self.fg_colour {
            has_updated = true;
            write!(f, "38;2;{r};{g};{b}")?;
        }


        if let Some(Colour { r, g, b }) = self.bg_colour {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "48;2;{r};{g};{b}")?;
        }


        if self.is_bold {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "1")?;
        }


        if self.is_dim {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "2")?;
        }


        if self.is_italic {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "3")?;
        }


        if self.is_underline {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "4")?;
        }


        if self.is_blinking {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "5")?;
        }


        if self.is_inverse {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "7")?;
        }


        if self.is_hidden {
            if has_updated { write!(f, ";")?; }
            has_updated = true;
            write!(f, "8")?;
        }


        if self.is_strikethrough {
            if has_updated { write!(f, ";")?; }
            write!(f, "9")?;
        }


        write!(f, "m")
    }
}


impl<T: core::fmt::Display> core::fmt::Display for ColouredString<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(not(no_std))]
        {
            let (no_colour, force_colour, is_terminal) = init_metadata();
            if !force_colour && (no_colour || !is_terminal) {
                return self.string.fmt(f)
            }
        }

        self.write_ansi(f)?;

        write!(f, "{}\u{001b}[0m", self.string)
    }
}


impl<T: core::fmt::Debug> core::fmt::Debug for ColouredString<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(not(no_std))]
        {
            let (no_colour, force_colour, is_terminal) = init_metadata();
            if !force_colour && (no_colour || !is_terminal) {
                return self.string.fmt(f)
            }
        }

        self.write_ansi(f)?;

        self.string.fmt(f)?;

        write!(f, "\u{001b}[0m")
    }
}


macro_rules! binding {
    ($($ident: ident)*) => {
        $(
            #[inline(always)]
            fn $ident(self) -> ColouredString<Self> {
                self.as_brush().$ident()
            }
        )*
    }
}


pub trait ColourBrush: Sized {
    #[inline(always)]
    fn as_brush(self) -> ColouredString<Self> {
        ColouredString {
            string: self,
            fg_colour: None, 
            bg_colour: None, 
            is_bold: false, 
            is_dim: false, 
            is_italic: false, 
            is_underline: false, 
            is_blinking: false, 
            is_inverse: false, 
            is_hidden: false, 
            is_strikethrough: false
        }
    }


    #[inline(always)]
    fn colour(self, colour: Colour) -> ColouredString<Self> {
        self.as_brush().colour(colour)
    }


    #[inline(always)]
    fn bg_colour(self, colour: Colour) -> ColouredString<Self> {
        self.as_brush().bg_colour(colour)
    }
    

    binding!(
        bold
        dim
        italic
        underline
        blinking
        inverse
        hidden
        strikethrough

        black 
        dark_grey 
        brown
        navy_blue
        olive_green
        teal
        red 
        green 
        blue 
        magenta 
        yellow 
        orange
        pink
        light_grey
        cyan 
        white 
        bg_dark_grey 
        bg_brown
        bg_navy_blue
        bg_olive_green
        bg_teal
        bg_red 
        bg_green 
        bg_blue 
        bg_magenta 
        bg_yellow 
        bg_orange
        bg_pink
        bg_light_grey
        bg_cyan 
        bg_white 
    );
}



macro_rules! colours {
    ($($name: ident : $r: literal $g: literal $b: literal),*) => {
        $(
            #[inline(always)]
            pub fn $name(self) -> Self { self.colour(Colour { r: $r, g: $g, b: $b }) }
        )*
    }
}


macro_rules! bg_colours {
    ($($name: ident : $r: literal $g: literal $b: literal),*) => {
        $(
            #[inline(always)]
            pub fn $name(self) -> Self { self.bg_colour(Colour { r: $r, g: $g, b: $b }) }
        )*
    }
}


impl<T> ColouredString<T> {
    colours! {
        black      :    0   0   0, 
        dark_grey  :  100 100 100, 
        brown      :  165  42  42,
        navy_blue  :    0   0 128,
        olive_green:  128 128   0,
        teal       :    0 128 128,
        red        :  255   0   0, 
        green      :    0 255   0, 
        blue       :    0   0 255, 
        magenta    :  255   0 255, 
        yellow     :  255 255   0, 
        orange     :  255 165   0,
        pink       :  255 192 203,
        light_grey :  200 200 200,
        cyan       :    0 255 255, 
        white      :  255 255 255
    }


    bg_colours! {
        bg_black      :    0   0   0, 
        bg_dark_grey  :  100 100 100, 
        bg_brown      :  165  42  42,
        bg_navy_blue  :    0   0 128,
        bg_olive_green:  128 128   0,
        bg_teal       :    0 128 128,
        bg_red        :  255   0   0, 
        bg_green      :    0 255   0, 
        bg_blue       :    0   0 255, 
        bg_magenta    :  255   0 255, 
        bg_yellow     :  255 255   0, 
        bg_orange     :  255 165   0,
        bg_pink       :  255 192 203,
        bg_light_grey :  200 200 200,
        bg_cyan       :    0 255 255, 
        bg_white      :  255 255 255
    }
}


impl<T> ColourBrush for T {}
