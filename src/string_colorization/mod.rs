use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Add;

use colored::{ColoredString, Colorize, Styles};
use itertools::Itertools;

use crate::alloc::string::ToString;
use crate::STYLES;

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
#[derive(Clone)]
pub struct Colorizer {
    /// Lettering color.
    ///
    /// Example: Applying ([foreground::Red]) to 'Red letters!' results in
    /// '<span style="color:red">Red letters!</span>', like:
    ///
    /// ````rust
    /// let red_foreground = foreground::Red.apply("Red foreground");
    /// println!("{red_foreground}");
    /// assert_eq!("[31mRed foreground[0m", red_foreground);
    /// ```
    foreground: Option<colored::Color>,
    /// Background color.
    ///
    /// Example: Applying [background::Red] to 'Red background!' results in
    /// '<span style="background-color:red;">Red background!</span>', like:
    ///
    /// ```rust
    /// let red_background = background::Red.apply("Red background");
    /// println!("{red_background}");
    /// assert_eq!("[41mRed background[0m", red_background);
    /// ```
    background: Option<colored::Color>,
    /// Stylizations applied to a text.
    ///
    /// Example: Applying [style::Italic]+[style::Bold] to 'Bold and italic' results in '***Bold and
    /// italic***', like:
    ///
    /// ```rust
    /// let bold_and_italic = (style::Italic+style::Bold).apply("Italic and bold");
    /// println!("{bold_and_italic}");
    /// assert_eq!("[3m[1mItalic and bold[0m[3m[0m", bold_and_italic);
    /// ```
    style_const: Option<u16>,
}

impl Add for Colorizer {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Colorizer::join_const(self, rhs)
    }
}

impl Colorizer {
    pub const fn new() -> Colorizer {
        Self { foreground: None, background: None, style_const: None }
    }

    pub const fn foreground(mut self, color: colored::Color) -> Colorizer {
        self.foreground = Some(color);
        self
    }

    pub const fn background(mut self, color: colored::Color) -> Colorizer {
        self.background = Some(color);
        self
    }

    pub const fn style(mut self, style: Styles) -> Colorizer {
        match style {
            Styles::Clear => self.style_const = Some(1 << crate::sytle_to_index(&Styles::Clear)),
            style => {
                if self.style_const.is_none() {
                    self.style_const = Some(0);
                }
                let this_style_const = match self.style_const {
                    None => unreachable!(),
                    Some(style_const) => style_const,
                };
                self.style_const = Some(this_style_const | (1 << crate::sytle_to_index(&style)));
            }
        }
        self
    }

    pub const fn join_const(mut old: Self, new: Self) -> Self {
        if new.foreground.is_some() {
            old.foreground = new.foreground;
        }
        if new.background.is_some() {
            old.background = new.background;
        }
        if old.style_const.is_some() && new.style_const.is_some() {
            let this_style_const = match old.style_const {
                None => { unreachable!() }
                Some(n) => { n }
            };
            let other_style_const = match new.style_const {
                None => { unreachable!() }
                Some(n) => { n }
            };

            let is_clear_style = (other_style_const & (1 << crate::sytle_to_index(&Styles::Clear))) == 1 << crate::sytle_to_index(&Styles::Clear);
            if is_clear_style {
                old.style_const = new.style_const;
                old.foreground = new.foreground;
                old.background = new.background;
            } else {
                old.style_const = Some(this_style_const | other_style_const);
            }
        } else {
            if new.style_const.is_some() {
                old.style_const = new.style_const;
            }
        }
        old
    }

    fn get_styles(&self) -> impl IntoIterator<Item=Styles> + '_ {
        STYLES.into_iter().filter(|style|
            self.style_const.is_some() && (self.style_const.unwrap() & (1 << crate::sytle_to_index(&style))) == 1 << crate::sytle_to_index(&style)
        )
    }

    pub fn styles<StyleT: Into<Styles>, StylesIter: IntoIterator<Item=StyleT>>(mut self, styles: StylesIter) -> Colorizer {
        for style in styles {
            self = self.style(style.into());
        }
        self
    }

    pub fn apply(&self, input: &str) -> String {
        let mut output = input.to_string();
        if let Some(background_color) = self.background {
            output = output.on_color(background_color).to_string();
        }
        if let Some(foreground_color) = self.foreground {
            output = output.color(foreground_color).to_string();
        }
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
            output = stylizer(ColoredString::from(output)).to_string().to_string();
        }
        output
    }
}


fn mem_dir_of_string(string: &str) -> (usize, usize) {
    let dir = unsafe { core::mem::transmute::<_, usize>(string.as_ptr()) };
    (dir, dir + string.len())
}

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
/// <span style="color:white">w</span>'*
///
/// ```rust
/// let rainbow = "Rainbow";
/// let colored_rainbow = crate::string_colorization::colorize(&rainbow, Some(foreground::White), [
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
/// from input, then the rule will just not be applied, for example, the following code prints
/// *'<span style="color:red">Red</span>, no red'*:
///
/// ``` rust
/// let string_to_colorize = "Red, no red";
/// let another_string = "Another string";
///
/// let colorized_string = crate::string_colorization::colorize(&string_to_colorize, None, [
///     (&string_to_colorize[0..3], foreground::Red), // This will turn 'Red' into red lettering
///     (&another_string[5..], foreground::Green),    // This is a substring to a different string
/// ]);                                               // from the input one (string_to_colorize),
///                                                   // meaning no changes will be applied, and
///                                                   // therefore, no text will turn green.
///
/// println!("{colorized_string}"); //Prints 'Red' in red coloring and 'no red' without color.
/// assert_eq!(colorized_string, r"[31mRed[0m, no red");
/// ```

pub fn colorize<'input, Modifiers: IntoIterator<Item=(&'input str, Colorizer)>>(input: &'input str, general_colorization: Option<Colorizer>, mut input_modifiers: Modifiers) -> String {
    if !colored::control::SHOULD_COLORIZE.should_colorize() {
        return input.to_string();
    }
    let (input_start, input_end) = mem_dir_of_string(input);
    let input_len = input.len();


    let input_modifiers = input_modifiers.into_iter();
    let tail = if let Some(general_colorization) = general_colorization {
        [(input, general_colorization)]
    } else { [("", Colorizer::new())] };
    let mut input_modifiers = tail.into_iter().chain(input_modifiers);


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

    let bounds = ranges_and_modifiers
        .iter()
        .flat_map(|(start, end, _)| [*start, *end])
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    let ranges_and_modifiers =
        bounds.windows(2)
            .map(|ran| (ran[0], ran[1]))
            .map(|(start, end)| {
                let mut colorization = Colorizer::new();
                for found_colorizer in ranges_and_modifiers.iter().filter(|range_and_modifier|
                    range_contains_other(start, end, range_and_modifier.0, range_and_modifier.1))
                    .map(|(_, _, modifier)| modifier) {
                    colorization = Colorizer::join_const(colorization, (*found_colorizer).clone());
                }
                (start, end, colorization)
            })
            .collect::<Vec<_>>();

    let mut output = input.to_string();
    ranges_and_modifiers.into_iter()
        .sorted_by(|(start_1, _, _), (start_2, _, _)| start_1.cmp(start_2).reverse())
        .for_each(|(start, offset_end, modifier)| {
            let modified = modifier.apply(&output[start..offset_end]);
            output = format!("{}{}{}", &output[..start], modified, &output[offset_end..]);
        });
    output
}