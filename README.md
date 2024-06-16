[![crates.io](https://img.shields.io/crates/v/string_colorization.svg)](https://crates.io/crates/string_colorization)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JorgeRicoVivas/string_colorization/rust.yml)](https://github.com/JorgeRicoVivas/string_colorization/actions)
[![docs.rs](https://img.shields.io/docsrs/string_colorization)](https://docs.rs/string_colorization/latest/string_colorization/)
![GitHub License](https://img.shields.io/github/license/JorgeRicoVivas/string_colorization)
[![GitHub License](https://img.shields.io/github/license/JorgeRicoVivas/string_colorization)](https://github.com/JorgeRicoVivas/string_colorization?tab=CC0-1.0-1-ov-file)


> *You are reading the documentation for string_colorization version 1.0.0*

Abstracts colorizing string from the [colored] crate by giving a struct [Colorizer] combining
foreground, background and stylizations to strings that can be applied later, and then uses them
on the [colorize] function to allow you to colorize a string given a series substring and
colorizers, for example, this code prints:
*<span style="background-color:lightgray">
<span style="color:red">R</span>
<span style="color:orange">a</span>
<span style="color:yellow">i</span>
<span style="color:green">n</span>
<span style="color:blue">b</span>
<span style="color:purple">o</span>
<span style="color:white">w</span>
</span>'*:
``` rust
colored::control::set_override(true); // Forces colorization,
                                      // this won't be necessary in your code.
use string_colorization::{background, foreground};

let rainbow = "Rainbow";
let default_colorizer = foreground::White+background::true_color(200,200,200);
let colored_rainbow = string_colorization::colorize(&rainbow, Some(default_colorizer), [
    (&rainbow[0..6], foreground::Red), // Turns 'Rainbo' into red letter, but since the rules
                                       // below override 'ainbo', only the 'R' results in
                                       // turning red.
    (&rainbow[1..6], foreground::true_color(255,160,0)), //Turns 'ainbo' into orange letters.
    (&rainbow[2..6], foreground::Yellow), // Turns 'inbo' into yellow.
    (&rainbow[3..6], foreground::Green),  // Turns 'nbo' into green.
    (&rainbow[4..6], foreground::Blue),   // Turns 'bo' into blue.
    (&rainbow[5..6], foreground::Magenta),// Turns 'o' into purple.
]);                                       // The letter 'n' wasn't reached by any of the other
                                          // patterns, meaning the 'general_colorization'
                                          // parameter will set its color, in this case, a white
                                          // lettering, if not indicated, it wouldn't colorize
                                          // the letter 'n', leaving it as plain.
println!("{colored_rainbow}");  //Prints Rainbow with colors
assert_eq!(colored_rainbow, r"[31m[48;2;200;200;200mR[0m[31m[0m[38;2;255;160;0m[48;2;200;200;200ma[0m[38;2;255;160;0m[0m[33m[48;2;200;200;200mi[0m[33m[0m[32m[48;2;200;200;200mn[0m[32m[0m[34m[48;2;200;200;200mb[0m[34m[0m[35m[48;2;200;200;200mo[0m[35m[0m[37m[48;2;200;200;200mw[0m[37m[0m");
```

If one of the rule's substring is a reference to another string different
from the *input* argument, then the rule will just not be applied, for example, the following
code prints *'<span style="color:red">Red</span>, no red'*:

``` rust
colored::control::set_override(true); // Forces colorization,
                                      // this won't be necessary in your code.
use string_colorization::foreground;

let string_to_colorize = "Red, no red";
let another_string = "Another string";
let colorized_string = string_colorization::colorize(&string_to_colorize, None, [
    (&string_to_colorize[0..3], foreground::Red), // This will turn 'Red' into red lettering
    (&another_string[5..], foreground::Green),    // This is a substring to a different string
]);                                               // from the input one (string_to_colorize),
                                                  // meaning no changes will be applied, and
                                                  // therefore, no text will turn green.

println!("{colorized_string}"); //Prints 'Red' in red coloring and 'no red' without color.
assert_eq!(colorized_string, r"[31mRed[0m, no red");
```

Find more information and examples in the function [colorize] and the struct [Colorizer].