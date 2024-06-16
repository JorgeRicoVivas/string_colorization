//#![cfg_attr(not(any(test, std)), no_std)]


extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::{Debug, Display, Formatter};
use std::collections::VecDeque;

use colored::Styles;
use itertools::Itertools;

use string_colorization::Colorizer;

use crate::alloc::string::ToString;

pub mod string_colorization;

#[derive(Debug, Default, Clone)]
struct ParsingError<'input> {
    where_: Option<&'input str>,
    error_detail: Option<Arc<dyn ParsingErrorDetail>>,
    start_point_of_error: Option<(usize, usize)>,
    end_point_of_error: Option<(usize, usize)>,
    causes: Vec<ParsingError<'input>>,
}

impl ParsingErrorDetail for Arc<dyn ParsingErrorDetail> {
    fn explain_error(&self) -> ErrorExplanation {
        (&**self).explain_error()
    }
}

impl<T: ParsingErrorDetail> ParsingErrorDetail for Arc<T> {
    fn explain_error(&self) -> ErrorExplanation {
        (&**self).explain_error()
    }
}

trait StringIsntEmpty {
    fn map_filter_empty(self) -> Option<String>;
}

impl<T: AsRef<str>> StringIsntEmpty for Option<T> {
    fn map_filter_empty(self) -> Option<String> {
        self.map(|string| string.as_ref().trim().to_string()).filter(|string| !string.is_empty())
    }
}

impl<'input, T: ParsingErrorDetail + 'static> From<T> for ParsingError<'input> {
    fn from(value: T) -> Self {
        ParsingError::new().error_detail(value)
    }
}

fn plural<Num, PrependToPlural, ToPluralize, OnEmpty>(n: Num, prepend: PrependToPlural, word_to_pluralize: ToPluralize, on_empty: OnEmpty) -> String
    where Num: Into<usize>, PrependToPlural: AsRef<str>, ToPluralize: AsRef<str>, OnEmpty: AsRef<str>

{
    let res = match n.into() {
        0 => return on_empty.as_ref().to_string(),
        1 => word_to_pluralize.as_ref().to_string(),
        n => format!("{n} {}s", word_to_pluralize.as_ref())
    };
    if prepend.as_ref().is_empty() {
        return res;
    }
    format!("{} {res}", prepend.as_ref())
}

impl<'input> ParsingError<'input> {
    pub fn new() -> Self {
        Self { where_: None, error_detail: None, start_point_of_error: None, end_point_of_error: None, causes: Vec::new() }
    }
    pub fn error_detail<ErrorDetail: ParsingErrorDetail + 'static>(mut self, error_detail: ErrorDetail) -> Self {
        self.error_detail = Some(Arc::new(error_detail));
        self
    }
    pub fn location_str(mut self, location_str: &'input str) -> Self {
        self.where_ = Some(location_str);
        self
    }
    pub fn start_point_of_error(mut self, line: usize, column: usize) -> Self {
        self.start_point_of_error = Some((line, column));
        self
    }
    pub fn end_point_of_error(mut self, line: usize, column: usize) -> Self {
        self.end_point_of_error = Some((line, column));
        self
    }

    pub fn with_cause<PError: Into<ParsingError<'input>>>(mut self, cause: PError) -> Self {
        self.add_cause(cause.into());
        self
    }

    pub fn without_causes(mut self) -> Self {
        self.causes = Vec::new();
        self
    }

    pub fn add_cause<PError: Into<ParsingError<'input>>>(&mut self, cause: PError) {
        self.causes.push(cause.into());
    }

    pub fn as_display_string(&self, is_displaying_as_cause_of_other: bool) -> Option<String> {
        let prepend_on_lines = if is_displaying_as_cause_of_other { "- ".to_string() } else { "".to_string() };
        let identation_on_extra_lines = if is_displaying_as_cause_of_other { 2 } else { 0 };

        let ErrorExplanation
        {
            complete_marker: general_colorizer,
            explanation: error_description,
            solution,
            colorization_markers: substring_colorizers
        }
            = self.error_detail.as_ref().map(|ast_error| ast_error.explain_error()).unwrap_or_default();
        let where_ = self.where_.clone()
            .map(|where_| string_colorization::colorize(where_, general_colorizer, substring_colorizers))
            .map_filter_empty();
        let location = match (self.start_point_of_error, self.end_point_of_error) {
            (Some((line_of_start, column_of_start)), Some((line_of_end, column_of_end))) =>
                Some(format!("From line {line_of_start} and column {column_of_start} to line {line_of_end} and column {column_of_end}")),
            (Some((line_of_start, column_of_start)), _) =>
                Some(format!("From line {line_of_start} and column {column_of_start}")),
            _ => None
        };
        let mut description = error_description.map_filter_empty()
            .map(|desc| indent::indent_by(identation_on_extra_lines, desc));
        let solution = solution
            .map(|desc| indent::indent_by(identation_on_extra_lines, desc));
        let causes = {
            if self.causes.is_empty() {
                None
            } else if self.causes.len() == 1 {
                Some(self.causes.get(0).unwrap().as_display_string(true)
                    .map(|displayed_cause| format!("Caused by:\n{displayed_cause}"))
                    .unwrap_or_else(|| "Caused by an unexplained error".to_string()))
            } else {
                let mut unexplained_causes = 0_usize;
                let causes = self.causes.iter()
                    .map(|cause| cause.as_display_string(true))
                    .filter(|cause| {
                        if cause.is_none() { unexplained_causes += 1 };
                        cause.is_some()
                    })
                    .map(|cause| cause.unwrap())
                    .sorted_by_key(|cause| cause.lines().count())
                    .collect::<Vec<_>>();
                let explained_causes = causes.len();

                let causes = match explained_causes {
                    0 => None,
                    1 => Some(causes.into_iter().next().unwrap()),
                    _ => Some(causes.into_iter().enumerate().map(|(index, cause)| {
                        format!("{prepend_on_lines}Cause nº{}:\n{cause}", index + 1)
                    }).join("\n"))
                }
                    .map_filter_empty();
                let unexplained_causes = Some(plural(unexplained_causes, "unexplained", "error", ""))
                    .map_filter_empty();

                let causes = match (causes, unexplained_causes) {
                    (Some(causes), Some(unexplained_causes)) => {
                        format!("Caused by {unexplained_causes} and the following {}:\n{causes}",
                                plural(explained_causes, "", "error", ""))
                    }
                    (Some(causes), None) => format!("Causes:\n{causes}"),
                    (None, Some(unexplained_causes)) => format!("Caused by {unexplained_causes}"),
                    (None, None) => unreachable!()
                };
                Some(causes)
            }
        }.map(|desc| indent::indent_by(identation_on_extra_lines, desc));
        if description.is_none() && [&where_, &location, &causes, &solution].iter().any(|other_display| other_display.is_some()) {
            description = Some("Unexplained error".to_string());
        }
        let mut displayed_error = [("At", where_), ("Reason", description), ("Solution", solution), ("Where", location), ("", causes)]
            .into_iter()
            .filter(|(_, string)| string.is_some())
            .map(|(prefix, line)| (prefix, line.unwrap()))
            .map(|(prefix, line)| if !prefix.is_empty() {
                let prefix = prefix.to_string() + ": ";
                let prefixed = format!("{prefix}{line}");
                indent::indent_by(prefix.len(), prefixed)
            } else { line })
            .map(|line| if is_displaying_as_cause_of_other { prepend_on_lines.clone() + &line } else { line })
            .join("\n");
        if displayed_error.trim().is_empty() {
            return None;
        }
        if is_displaying_as_cause_of_other {
            displayed_error = indent::indent_all_by(2, displayed_error);
        }
        Some(displayed_error)
    }

    pub fn final_errors(&self) -> Vec<&Self> {
        if self.causes.is_empty() {
            return vec![&self];
        } else {
            self.causes.iter().flat_map(|cause| cause.final_errors()).collect()
        }
    }

    pub fn reverse_errors(&self) -> Vec<ParsingError<'input>> {
        let mut result = Default::default();
        let mut current_stack = Default::default();
        self.revese_errors_int(&mut result, &mut current_stack);

        result.into_iter().map(|mut errors_stack| {
            let mut reverse_error = errors_stack.remove(errors_stack.len() - 1).clone().without_causes();
            while !errors_stack.is_empty() {
                reverse_error = errors_stack.remove(errors_stack.len() - 1).clone().without_causes().with_cause(reverse_error);
            }
            reverse_error
        }).collect::<Vec<_>>()
    }

    fn revese_errors_int<'selflf>(&'selflf self, result: &mut Vec<Vec<&'selflf Self>>, current_stack: &mut VecDeque<&'selflf Self>) {
        current_stack.push_front(self);
        if self.causes.is_empty() {
            result.push(current_stack.clone().into_iter().collect());
        } else {
            self.causes.iter().for_each(|cause| cause.revese_errors_int(result, current_stack));
        }
        current_stack.pop_front();
    }
}

impl<'input> Display for ParsingError<'input> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.as_display_string(false).unwrap_or_else(|| "Unexplained error".to_string()))
    }
}


#[cfg(all(std, not(test)))]
impl<'input> core::error::Error for ParsingError<'input> {}

#[derive(Default)]
pub struct ErrorExplanation<'input> {
    explanation: Option<String>,
    solution: Option<String>,
    complete_marker: Option<Colorizer>,
    colorization_markers: Vec<(&'input str, Colorizer)>,
}

impl<'input> ErrorExplanation<'input> {
    pub fn new() -> Self {
        Self { explanation: None, solution: None, colorization_markers: Vec::new(), complete_marker: None }
    }

    pub fn explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation)
            .map(|explanation| explanation.trim().to_string())
            .filter(|explanation| !explanation.is_empty());
        self
    }

    pub fn solution(mut self, solution: String) -> Self {
        self.solution = Some(solution)
            .map(|solution| solution.trim().to_string())
            .filter(|solution| !solution.is_empty());
        self
    }

    pub fn complete_input_colorization(mut self, complete_marker: Colorizer) -> Self {
        self.complete_marker = Some(complete_marker);
        self
    }

    pub fn colorization_markers<Color, Input, MarkerIterator>(mut self, colorization_markers: MarkerIterator) -> Self
        where Color: Into<Colorizer>,
              Input: Into<&'input str>,
              MarkerIterator: IntoIterator<Item=(Input, Color)> {
        self.colorization_markers.extend(colorization_markers.into_iter().map(|(input, color)| (input.into(), color.into())));
        self
    }

    pub fn colorization_marker(mut self, string: &'input str, colorization: Colorizer) -> Self {
        self.colorization_markers.push((string, colorization));
        self
    }
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


trait ParsingErrorDetail: Debug {
    fn explain_error(&self) -> ErrorExplanation;

    fn location_str(self, where_: &str) -> ParsingError where Self: Sized + 'static {
        ParsingError::new().error_detail(self).location_str(where_)
    }
    fn start_point_of_error<'input>(self, line: usize, column: usize) -> ParsingError<'input> where Self: Sized + 'static {
        ParsingError::new().error_detail(self).end_point_of_error(line, column)
    }
    fn end_point_of_error<'input>(self, line: usize, column: usize) -> ParsingError<'input> where Self: Sized + 'static {
        ParsingError::new().error_detail(self).end_point_of_error(line, column)
    }

    fn to_parsing_error<'input>(self) -> ParsingError<'input> where Self: Sized + 'static {
        ParsingError::new().error_detail(self)
    }
}

fn main() {}

/*
pub mod example;

fn main() {
    example::main();
}
*/