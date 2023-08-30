# Colourful
A lightweight, fast, and easy-to-use library for creating coloured strings with ANSI  

Ergonomics and speed should not be mutually exclusive, let's look at some examples
```rs
println!("'{}'", "bold red on navy blue".red().bold().bg_navy_blue());
println!("'{}'", "green background".bg_colour(Colour::rgb(0, 255, 0,)));
println!("'{}'", "red on green background".colour(Colour::rgb(255, 0, 0)).bg_colour(Colour::rgb(0, 255, 0,)));
println!("'{}'", "bold".bold());
println!("'{}'", "dim".dim());
println!("'{}'", "italic".italic());
println!("'{}'", "underline".underline());
println!("'{}'", "blinking".blinking());
println!("'{}'", "inverse".inverse());
println!("'{}'", "hidden".hidden());
println!("'{}''", "strikethrough".strikethrough());

println!("also works on any type with display {}", Vector3(0.5, 0.3, 1.2).red().blinking());
println!("or debug.. who am i to judge {:#?}", vec!["mhm", "sup", "okay"].strikethrough());
```

# Features
- No other dependancies
- Allocation free
- *Blazingly fast*
- Does not emit colours when piping to a file
- Respects `NO_COLOR` and `FORCE_COLOR`
- Ability to colour and style any type that implements `Display` or `Debug`


# Usage
1) Add the dependancy to your crate
2) Import `colourful::ColourBrush`
3) You're good to go!


## no_std
the base crate relies on reading environment variables to abide by `NO_COLOR`, `FORCE_COLOR` and 
not print colours when piping. the crate should be able to compile with `no_std` without you needing
to do anything, but these features won't be available