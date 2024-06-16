#![no_std]

//! [![crates.io](https://img.shields.io/crates/v/string_colorization.svg)](https://crates.io/crates/string_colorization)
//! [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JorgeRicoVivas/string_colorization/rust.yml)](https://github.com/JorgeRicoVivas/string_colorization/actions)
//! [![docs.rs](https://img.shields.io/docsrs/string_colorization)](https://docs.rs/string_colorization/latest/string_colorization/)
//! ![GitHub License](https://img.shields.io/github/license/JorgeRicoVivas/string_colorization)
//! [![GitHub License](https://img.shields.io/github/license/JorgeRicoVivas/string_colorization)](https://github.com/JorgeRicoVivas/string_colorization?tab=CC0-1.0-1-ov-file)
//! > *You are reading the documentation for string_colorization version 1.0.0*
//!
//! Abstracts colorizing string from the [colored] crate by giving a struct [Colorizer] combining
//! foreground, background and stylizations to strings that can be applied later, and then uses them
//! on the [colorize] function to allow you to colorize a string given a series substring and
//! colorizers, for example, this code prints:
//! *<span style="background-color:lightgray">
//! <span style="color:red">R</span>
//! <span style="color:orange">a</span>
//! <span style="color:yellow">i</span>
//! <span style="color:green">n</span>
//! <span style="color:blue">b</span>
//! <span style="color:purple">o</span>
//! <span style="color:white">w</span>
//! </span>'*:
//! ```rust
//! colored::control::set_override(true); // Forces colorization,
//!                                       // this won't be necessary in your code.
//! use string_colorization::{background, foreground};
//!
//! let rainbow = "Rainbow";
//! let default_colorizer = foreground::White+background::true_color(200,200,200);
//! let colored_rainbow = string_colorization::colorize(&rainbow, Some(default_colorizer), [
//!     (&rainbow[0..6], foreground::Red), // Turns 'Rainbo' into red letter, but since the rules
//!                                        // below override 'ainbo', only the 'R' results in
//!                                        // turning red.
//!     (&rainbow[1..6], foreground::true_color(255,160,0)), //Turns 'ainbo' into orange letters.
//!     (&rainbow[2..6], foreground::Yellow), // Turns 'inbo' into yellow.
//!     (&rainbow[3..6], foreground::Green),  // Turns 'nbo' into green.
//!     (&rainbow[4..6], foreground::Blue),   // Turns 'bo' into blue.
//!     (&rainbow[5..6], foreground::Magenta),// Turns 'o' into purple.
//! ]);                                       // The letter 'n' wasn't reached by any of the other
//!                                           // patterns, meaning the 'general_colorization'
//!                                           // parameter will set its color, in this case, a white
//!                                           // lettering, if not indicated, it wouldn't colorize
//!                                           // the letter 'n', leaving it as plain.
//! println!("{colored_rainbow}");  //Prints Rainbow with colors
//! assert_eq!(colored_rainbow, r"[31m[48;2;200;200;200mR[0m[31m[0m[38;2;255;160;0m[48;2;200;200;200ma[0m[38;2;255;160;0m[0m[33m[48;2;200;200;200mi[0m[33m[0m[32m[48;2;200;200;200mn[0m[32m[0m[34m[48;2;200;200;200mb[0m[34m[0m[35m[48;2;200;200;200mo[0m[35m[0m[37m[48;2;200;200;200mw[0m[37m[0m");
//! ```
//!
//! If one of the rule's substring is a reference to another string different
//! from the *input* argument, then the rule will just not be applied, for example, the following
//! code prints *'<span style="color:red">Red</span>, no red'*:
//!
//! ``` rust
//! colored::control::set_override(true); // Forces colorization,
//!                                       // this won't be necessary in your code.
//! use string_colorization::foreground;
//!
//! let string_to_colorize = "Red, no red";
//! let another_string = "Another string";
//! let colorized_string = string_colorization::colorize(&string_to_colorize, None, [
//!     (&string_to_colorize[0..3], foreground::Red), // This will turn 'Red' into red lettering
//!     (&another_string[5..], foreground::Green),    // This is a substring to a different string
//! ]);                                               // from the input one (string_to_colorize),
//!                                                   // meaning no changes will be applied, and
//!                                                   // therefore, no text will turn green.
//!
//! println!("{colorized_string}"); //Prints 'Red' in red coloring and 'no red' without color.
//! assert_eq!(colorized_string, r"[31mRed[0m, no red");
//! ```
//!
//! Find more information and examples in the function [colorize] and the struct [Colorizer].

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ops::Add;

use colored::*;

macro_rules! make_colors {
        ($function:ident $($color:ident),*) => {
            $(
                #[doc = stringify!($color)]
                #[doc = stringify!($function)]
                #[doc = " colorizer"]
                #[allow(non_upper_case_globals)]
                pub const $color:Colorizer=Colorizer::new().$function(colored::Color::$color);
            )*
        };
    }

/// Constants for creating foreground [Colorizer]s
pub mod foreground {
    use super::Colorizer;

    make_colors! {foreground Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, BrightBlack, BrightRed, BrightGreen, BrightYellow, BrightBlue, BrightMagenta, BrightCyan, BrightWhite}

    pub const fn true_color(red: u8, green: u8, blue: u8) -> Colorizer {
        Colorizer::new().foreground(colored::Color::TrueColor { r: red, g: green, b: blue })
    }
}

/// Constants for creating background [Colorizer]s
pub mod background {
    use super::Colorizer;

    make_colors! {background Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, BrightBlack,
        BrightRed, BrightGreen, BrightYellow, BrightBlue, BrightMagenta, BrightCyan, BrightWhite}

    /// Creates a background [Colorizer] which will set the background of some text according to the
    /// Red, Green and Blue values given
    pub const fn true_color(red: u8, green: u8, blue: u8) -> Colorizer {
        Colorizer::new().background(colored::Color::TrueColor { r: red, g: green, b: blue })
    }
}

/// Constants for creating stylized [Colorizer]s
pub mod style {
    use super::Colorizer;

    macro_rules! make_styles {
        ($($style:ident),*) => {
            $(
                #[doc = stringify!($style)]
                #[doc = "styled colorizer"]
                #[allow(non_upper_case_globals)]
                pub const $style:Colorizer=Colorizer::new().style(colored::Styles::$style);
            )*
        };
    }

    make_styles!(Clear, Bold, Dimmed, Underline, Reversed, Italic, Blink, Hidden, Strikethrough);
}

/// Defines a foreground, background, and styles that can be appied on a string to format it.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Colorizer {
    /// Lettering color.
    ///
    /// Example: Applying ([foreground::Red]) to 'Red letters!' results in
    /// '<span style="color:red">Red letters!</span>', like:
    ///
    /// ```rust
    /// colored::control::set_override(true); // Forces colorization,
    ///                                       // this won't be necessary in your code.
    /// use string_colorization::foreground;
    ///
    /// let red_foreground = foreground::Red.apply("Red foreground");
    /// println!("{red_foreground}");
    /// assert_eq!("[31mRed foreground[0m", red_foreground);
    /// ```
    foreground: Option<Color>,
    /// Background color.
    ///
    /// Example: Applying [background::Red] to 'Red background!' results in
    /// '<span style="background-color:red;">Red background!</span>', like:
    ///
    /// ```rust
    /// colored::control::set_override(true); // Forces colorization,
    ///                                       // this won't be necessary in your code.
    /// use string_colorization::background;
    ///
    /// let red_background = background::Red.apply("Red background");
    /// println!("{red_background}");
    /// assert_eq!("[41mRed background[0m", red_background);
    /// ```
    background: Option<Color>,
    /// Stylizations applied to a text.
    ///
    /// Example: Applying [style::Italic]+[style::Bold] to 'Bold and italic' results in '***Bold and
    /// italic***', like:
    ///
    /// ```rust
    /// colored::control::set_override(true); // Forces colorization,
    ///                                       // this won't be necessary in your code.
    /// use string_colorization::style;
    ///
    /// let bold_and_italic = (style::Italic+style::Bold).apply("Italic and bold");
    /// println!("{bold_and_italic}");
    /// assert_eq!("[3m[1mItalic and bold[0m[3m[0m", bold_and_italic);
    /// ```
    style_const: Option<u16>,
}

const STYLES: [Styles; 9] = [Styles::Clear, Styles::Bold, Styles::Dimmed, Styles::Underline,
    Styles::Reversed, Styles::Italic, Styles::Blink, Styles::Hidden, Styles::Strikethrough];

const fn sytle_to_index(style: &Styles) -> usize {
    match style {
        Styles::Clear => 0,
        Styles::Bold => 1,
        Styles::Dimmed => 2,
        Styles::Underline => 3,
        Styles::Reversed => 4,
        Styles::Italic => 5,
        Styles::Blink => 6,
        Styles::Hidden => 7,
        Styles::Strikethrough => 8,
    }
}


/// Allows to join two [Colorizer]s, where the second one of the sum has precedence.
///
/// This is the same as applying [Colorizer::join_with], meaning the documentation here is the same,
/// but using *'join_with'* instead of the *'+'* operator.
///
/// - Example: This results into a string with a blue background and green lettering:
///
/// ```rust
/// use colored::Color;
/// use string_colorization::Colorizer;
/// colored::control::set_override(true); //Forces colorization
///                                       //this won't be necessary in your code.
///
/// let blue_background = Colorizer::new().background(Color::Blue);
/// let green_foreground = Colorizer::new().foreground(Color::Green);
/// let blue_bg_and_green_fg = blue_background+green_foreground;
/// let output_string = blue_bg_and_green_fg.apply("Blue background with green letters!");
/// println!("{output_string}"); //Prints some text with Blue background and green letters
///
/// assert_eq!(output_string, "[32m[44mBlue background with green letters![0m[32m[0m");
/// let manually_created = Colorizer::new().background(Color::Blue).foreground(Color::Green);
/// assert_eq!(manually_created, blue_bg_and_green_fg);
/// ```
///
/// - This makes much more comfortable to use the constants colorizers from the [foreground],
/// [background] and [style] modules, for example, the blue background and green lettering
/// [Colorizer] could also have been created this way:
///
/// ```rust
/// use colored::Color;
/// use string_colorization::{background, Colorizer, foreground};
///
/// let applying_sum = background::Blue+foreground::Green;
/// let manually_created = Colorizer::new().background(Color::Blue).foreground(Color::Green);
/// assert_eq!(applying_sum, manually_created);
/// ```
///
/// - When summing two colorizers, the second one has precedence, for example, summing one with blue
/// letters to one with green letters results into a colorizer with just green letters:
///
/// ```rust
/// use colored::Color;
/// use string_colorization::{Colorizer, foreground};
///
/// let green_from_blue_and_green_sum = foreground::Blue+foreground::Green;
/// assert_eq!(green_from_blue_and_green_sum, foreground::Green);
/// ```
///
/// - The reason for making the second one to have precedence over the first one instead of the first
/// one is as it should result in the same order as when using the builder pattern, where applying
/// [Colorizer::foreground] to [Color::Blue] and then to [Color::Green], results into just Green
/// coloring:
///
/// ```rust
/// use colored::Color;
/// use string_colorization::{Colorizer, foreground};
///
/// let applying_sum = foreground::Blue+foreground::Green;
/// let manually_created = Colorizer::new().foreground(Color::Blue).foreground(Color::Green);
/// assert_eq!(applying_sum, manually_created);
/// ```

impl Add for Colorizer {

    /// Summing two [Colorizer]s results into a new [Colorizer].
    type Output = Self;

    /// Joins two [Colorizer]s together, where the second one of the sum has precedence.
    fn add(self, rhs: Self) -> Self::Output {
        self.join_with(rhs)
    }
}

impl Colorizer {

    /// Creates a new Colorizer where no foreground, background or style has been set.
    pub const fn new() -> Colorizer {
        Self { foreground: None, background: None, style_const: None }
    }

    /// Sets this [Colorizer] to make letters to turn into the color indicated by parameter.
    ///
    /// Example: Applying this [Colorizer] to 'Red letters!' results in
    /// '<span style="color:red">Red letters!</span>':
    ///
    /// ```rust
    /// colored::control::set_override(true); // Forces colorization,
    ///                                       // this won't be necessary in your code.
    /// use string_colorization::Colorizer;
    /// use colored::Color;
    ///
    /// let red_foreground = Colorizer::new().foreground(Color::Red).apply("Red foreground");
    /// println!("{red_foreground}");
    /// assert_eq!("[31mRed foreground[0m", red_foreground);
    /// ```
    pub const fn foreground(mut self, color: Color) -> Colorizer {
        self.foreground = Some(color);
        self
    }

    /// Sets this [Colorizer] to make backgrounds of letters to turn into the color indicated by
    /// parameter.
    ///
    /// Example: Applying this [Colorizer] to 'Red background!' results in
    /// '<span style="background-color:red;">Red background!</span>':
    ///
    /// ```rust
    /// colored::control::set_override(true); // Forces colorization,
    ///                                       // this won't be necessary in your code.
    /// use string_colorization::Colorizer;
    /// use colored::Color;
    ///
    /// let red_background = Colorizer::new().background(Color::Red).apply("Red background");
    /// println!("{red_background}");
    /// assert_eq!("[41mRed background[0m", red_background);
    /// ```
    pub const fn background(mut self, color: Color) -> Colorizer {
        self.background = Some(color);
        self
    }

    /// Sets this [Colorizer] to make stylization of letters to the ones indicated by  parameter.
    ///
    /// Example: Applying this [Colorizer] to 'Bold and italic' results in '***Bold and
    /// italic***':
    ///
    /// ```rust
    /// colored::control::set_override(true); // Forces colorization,
    ///                                       // this won't be necessary in your code.
    /// use string_colorization::Colorizer;
    /// use colored::{Color, Styles};
    ///
    /// let bold_and_italic = Colorizer::new().style(Styles::Italic).style(Styles::Bold)
    ///         .apply("Italic and bold");
    /// println!("{bold_and_italic}");
    /// assert_eq!("[3m[1mItalic and bold[0m[3m[0m", bold_and_italic);
    /// ```
    pub const fn style(mut self, style: Styles) -> Colorizer {
        match style {
            Styles::Clear => {
                self.style_const = Some(1 << sytle_to_index(&Styles::Clear));
                self.foreground = None;
                self.background = None;
            }
            style => {
                if self.style_const.is_none() {
                    self.style_const = Some(0);
                }
                let this_style_const = match self.style_const {
                    None => unreachable!(),
                    Some(style_const) => style_const,
                };
                self.style_const = Some(this_style_const | (1 << sytle_to_index(&style)));
            }
        }
        self
    }

    /// Allows to join two [Colorizer]s, where the second one of the sum has precedence.
    ///
    /// - Example: This results into a string with a blue background and green lettering:
    ///
    /// ```rust
    /// use colored::Color;
    /// use string_colorization::Colorizer;
    /// colored::control::set_override(true); //Forces colorization
    ///                                       //this won't be necessary in your code.
    ///
    /// let blue_background = Colorizer::new().background(Color::Blue);
    /// let green_foreground = Colorizer::new().foreground(Color::Green);
    /// let blue_bg_and_green_fg = blue_background.join_with(green_foreground);
    /// let output_string = blue_bg_and_green_fg.apply("Blue background with green letters!");
    /// println!("{output_string}"); //Prints some text with Blue background and green letters
    ///
    /// assert_eq!(output_string, "[32m[44mBlue background with green letters![0m[32m[0m");
    /// let manually_created = Colorizer::new().background(Color::Blue).foreground(Color::Green);
    /// assert_eq!(manually_created, blue_bg_and_green_fg);
    /// ```
    ///
    /// - This makes much more comfortable to use the constants colorizers from the [foreground],
    /// [background] and [style] modules, for example, the blue background and green lettering
    /// [Colorizer] could also have been created this way:
    ///
    /// ```rust
    /// use colored::Color;
    /// use string_colorization::{background, Colorizer, foreground};
    ///
    /// let applying_sum = background::Blue.join_with(foreground::Green);
    /// let manually_created = Colorizer::new().background(Color::Blue).foreground(Color::Green);
    /// assert_eq!(applying_sum, manually_created);
    /// ```
    ///
    /// - When summing two colorizers, the second one has precedence, for example, summing one with blue
    /// letters to one with green letters results into a colorizer with just green letters:
    ///
    /// ```rust
    /// use colored::Color;
    /// use string_colorization::{Colorizer, foreground};
    ///
    /// let green_from_blue_and_green_sum = foreground::Blue.join_with(foreground::Green);
    /// assert_eq!(green_from_blue_and_green_sum, foreground::Green);
    /// ```
    ///
    /// - The reason for making the second one to have precedence over the first one instead of the first
    /// one is as it should result in the same order as when using the builder pattern, where applying
    /// [Colorizer::foreground] to [Color::Blue] and then to [Color::Green], results into just Green
    /// coloring:
    ///
    /// ```rust
    /// use colored::Color;
    /// use string_colorization::{Colorizer, foreground};
    ///
    /// let applying_sum = foreground::Blue.join_with(foreground::Green);
    /// let manually_created = Colorizer::new().foreground(Color::Blue).foreground(Color::Green);
    /// assert_eq!(applying_sum, manually_created);
    /// ```
    pub const fn join_with(mut self, new: Self) -> Self {
        if new.foreground.is_some() {
            self.foreground = new.foreground;
        }
        if new.background.is_some() {
            self.background = new.background;
        }
        if self.style_const.is_some() && new.style_const.is_some() {
            let this_style_const = match self.style_const {
                None => { unreachable!() }
                Some(n) => { n }
            };
            let other_style_const = match new.style_const {
                None => { unreachable!() }
                Some(n) => { n }
            };

            let is_clear_style = (other_style_const & (1 << sytle_to_index(&Styles::Clear))) == 1 << sytle_to_index(&Styles::Clear);
            if is_clear_style {
                self.style_const = new.style_const;
                self.foreground = new.foreground;
                self.background = new.background;
            } else {
                self.style_const = Some(this_style_const | other_style_const);
            }
        } else {
            if new.style_const.is_some() {
                self.style_const = new.style_const;
            }
        }
        self
    }

    /// Transforms all the styles in [Colorizer::style_const] to [Styles].
    fn get_styles(&self) -> impl IntoIterator<Item=Styles> + '_ {
        STYLES.into_iter().filter(|style|
            self.style_const.is_some() && (self.style_const.unwrap() & (1 << sytle_to_index(&style))) == 1 << sytle_to_index(&style)
        )
    }

    /// Adds the following styles to this [Colorizer], meaning this is the same as applying
    /// [Colorizer::style] on all of them, for example, both here result in the same:
    ///
    /// ```rust
    /// use colored::Styles;
    /// use string_colorization::Colorizer;
    ///
    /// let using_styles = Colorizer::new().styles([Styles::Bold, Styles::Italic]);
    /// let using_style = Colorizer::new().style(Styles::Bold).style(Styles::Italic);
    ///
    /// assert_eq!(using_styles, using_style);
    /// ```
    pub fn styles<StyleT: Into<Styles>, StylesIter: IntoIterator<Item=StyleT>>(mut self, styles: StylesIter) -> Colorizer {
        for style in styles {
            self = self.style(style.into());
        }
        self
    }

    /// Applies the foreground color, background color, and style to an owned copy of the input
    /// string, and the returns it after applying them, leaving the input intact.
    pub fn apply(&self, input: &str) -> String {
        let mut output = input.to_string();
        for style in self.get_styles() {
            let stylizer: fn(ColoredString) -> ColoredString = match style {
                Styles::Clear => Colorize::clear,
                Styles::Bold => Colorize::bold,
                Styles::Dimmed => Colorize::dimmed,
                Styles::Underline => Colorize::underline,
                Styles::Reversed => Colorize::reversed,
                Styles::Italic => Colorize::italic,
                Styles::Blink => Colorize::blink,
                Styles::Hidden => Colorize::hidden,
                Styles::Strikethrough => Colorize::strikethrough,
            };
            output = stylizer(ColoredString::from(output)).to_string();
        }
        if let Some(background_color) = self.background {
            output = output.on_color(background_color).to_string();
        }
        if let Some(foreground_color) = self.foreground {
            output = output.color(foreground_color).to_string();
        }
        output
    }
}

/// Given a str, it returns the memory address it is located at, and then the final position in
/// memory taken by this str
fn mem_dir_of_string(string: &str) -> (usize, usize) {
    let dir = unsafe { core::mem::transmute::<_, usize>(string.as_ptr()) };
    (dir, dir + string.len())
}

/// Checks if range 1 contains the second one, being partially or completely, for example: 2..4
/// completely contains 2..3, 2..4 and 3..4, while it partially contains ranges like 3..5 (in 3..4)
/// or 1..3 (in 2..3), but does not contain nor partially nor completely a range like 10..20 or
/// 4..5.
fn range_contains_other(range_1_start: usize, range_1_end: usize, range_2_start: usize, range_2_end: usize) -> bool {
    let res = range_2_end > range_1_start && range_2_start < range_1_end;
    let res = res;
    res
}

/// Colorizes every substring over a string and returns a [String] where every substring has been
/// stylized according to these rules.
///
/// * `input` - Text whose substrings we want to colorize.
/// * `general_colorization` - Colorization to apply when no rule applies to a character.
/// * `input_modifiers` - Iterator of substring and colorization pairs, this is: a rule, for every
/// rule it searches for the substring over the original string, and then applies it's colorization
///
/// When two or more substrings inside the `input_modifiers` parameter are substring of the same
/// characters, the last [Colorizer]s, for example, the following code prints:
/// *'<span style="color:red">R</span>
/// <span style="color:orange">a</span>
/// <span style="color:yellow">i</span>
/// <span style="color:green">n</span>
/// <span style="color:blue">b</span>
/// <span style="color:purple">o</span>
/// <span style="color:gray">w</span>'*
///
/// ```rust
/// colored::control::set_override(true); // Forces colorization,
///                                       // this won't be necessary in your code.
/// use string_colorization::foreground;
///
/// let rainbow = "Rainbow";
/// let colored_rainbow = string_colorization::colorize(&rainbow, Some(foreground::White), [
///     (&rainbow[0..6], foreground::Red), // Turns 'Rainbo' into red letter, but since the rules
///                                        // below override 'ainbo', only the 'R' results in
///                                        // turning red.
///     (&rainbow[1..6], foreground::true_color(255,160,0)), //Turns 'ainbo' into orange letters.
///     (&rainbow[2..6], foreground::Yellow), // Turns 'inbo' into yellow.
///     (&rainbow[3..6], foreground::Green),  // Turns 'nbo' into green.
///     (&rainbow[4..6], foreground::Blue),   // Turns 'bo' into blue.
///     (&rainbow[5..6], foreground::Magenta),// Turns 'o' into purple.
/// ]);                                       // The letter 'n' wasn't reached by any of the other
///                                           // patterns, meaning the 'general_colorization'
///                                           // parameter will set its color, in this case, a white
///                                           // lettering, if not indicated, it wouldn't colorize
///                                           // the letter 'n', leaving it as plain.
/// println!("{colored_rainbow}");  //Prints Rainbow with colors
/// assert_eq!(colored_rainbow, r"[31mR[0m[38;2;255;160;0ma[0m[33mi[0m[32mn[0m[34mb[0m[35mo[0m[37mw[0m");
/// ```
///
/// * *IMPORTANT NOTE*: If one of the rule's substring is a reference to another string different
/// from the *input* argument, then the rule will just not be applied, for example, the following
/// code prints *'<span style="color:red">Red</span>, no red'*:
///
/// ``` rust
/// colored::control::set_override(true); // Forces colorization,
///                                       // this won't be necessary in your code.
/// use string_colorization::foreground;
///
/// let string_to_colorize = "Red, no red";
/// let another_string = "Another string";
/// let colorized_string = string_colorization::colorize(&string_to_colorize, None, [
///     (&string_to_colorize[0..3], foreground::Red), // This will turn 'Red' into red lettering
///     (&another_string[5..], foreground::Green),    // This is a substring to a different string
/// ]);                                               // from the input one (string_to_colorize),
///                                                   // meaning no changes will be applied, and
///                                                   // therefore, no text will turn green.
///
/// println!("{colorized_string}"); //Prints 'Red' in red coloring and 'no red' without color.
/// assert_eq!(colorized_string, r"[31mRed[0m, no red");
/// ```

pub fn colorize<'input, Modifiers: IntoIterator<Item=(&'input str, Colorizer)>>(input: &'input str, general_colorization: Option<Colorizer>, input_modifiers: Modifiers) -> String {
    if !colored::control::SHOULD_COLORIZE.should_colorize() {
        return input.to_string();
    }
    let (input_start, input_end) = mem_dir_of_string(input);
    let input_len = input.len();
    let input_modifiers = input_modifiers.into_iter();

    let tail = if let Some(general_colorization) = general_colorization {
        [(input, general_colorization)]
    } else { [("", Colorizer::new())] };
    let input_modifiers = tail.into_iter().chain(input_modifiers);

    let ranges_and_modifiers = input_modifiers.into_iter()
        .map(|(str_slice, value)| {
            let (offset_start, offset_end) = mem_dir_of_string(str_slice);
            (offset_start, offset_end, value)
        })
        .filter(|(offset_start, offset_end, _)| {
            range_contains_other(*offset_start, *offset_end, input_start, input_end)
        })
        .map(|(offset_start, offset_end, value)| {
            (
                offset_start.checked_sub(input_start).unwrap_or(0).min(input_len),
                offset_end.checked_sub(input_start).unwrap_or(0).min(input_len),
                value
            )
        })
        .filter(|(start, end, _)| end > start)
        .collect::<Vec<_>>();

    let mut bounds = ranges_and_modifiers
        .iter()
        .flat_map(|(start, end, _)| [*start, *end])
        .collect::<Vec<_>>();
    bounds.sort();
    bounds.dedup();


    let mut ranges_and_modifiers =
        bounds.windows(2)
            .map(|ran| (ran[0], ran[1]))
            .map(|(start, end)| {
                let mut colorization = Colorizer::new();
                for found_colorizer in ranges_and_modifiers.iter().filter(|range_and_modifier|
                    range_contains_other(start, end, range_and_modifier.0, range_and_modifier.1))
                    .map(|(_, _, modifier)| modifier) {
                    colorization = colorization.join_with(found_colorizer.clone());
                }
                (start, end, colorization)
            })
            .collect::<Vec<_>>();
    ranges_and_modifiers.sort_by(|(start_1, _, _), (start_2, _, _)| start_1.cmp(start_2).reverse());

    let mut output = input.to_string();
    ranges_and_modifiers.into_iter()
        .for_each(|(start, offset_end, modifier)| {
            let modified = modifier.apply(&output[start..offset_end]);
            output = format!("{}{}{}", &output[..start], modified, &output[offset_end..]);
        });
    output
}