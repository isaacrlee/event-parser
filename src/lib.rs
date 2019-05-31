use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};
use regex::RegexSet;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DateParseError {
    DateUnknown,
    IoError(std::io::Error),
}

impl fmt::Display for DateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateParseError::DateUnknown => write!(f, "Error: Date unknown"),
            DateParseError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl Error for DateParseError {
    fn description(&self) -> &str {
        "Date unknown"
    }
}

#[derive(Debug)]
pub enum TimeParseError {
    TimeUnknown,
    IoError(std::io::Error),
}

impl fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeParseError::TimeUnknown => write!(f, "Error: Time unknown"),
            TimeParseError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl Error for TimeParseError {
    fn description(&self) -> &str {
        "Time unknown"
    }
}

// Examples
// (12pm, 12, noon, twelve, at 12, 10:30, 12:30pm}
// {Saturday, 6/1, sat, this saturday, next saturday, last saturday, june 1, june 1st}
// {tonight, last night, tomorrow night, tomorrow morning, lunch, dinner, breakfast, dawn, late, afternoon, evening, now, in two hours, midnight}

// Parser

// Date Parser
pub struct DateParser {}
impl DateParser {
    pub fn parse(&self, text: &str) -> Result<NaiveDate, DateParseError> {
        self.parse_relative(text, Some(&Utc::now().date().naive_utc()))
    }

    pub fn parse_relative(
        &self,
        text: &str,
        now: Option<&NaiveDate>,
    ) -> Result<NaiveDate, DateParseError> {
        unimplemented!()
        // DateExpr.recognize(text)

        // match DateExpr
        // create Date based on DateExpr and now
    }
}

// Time Parser
pub struct TimeParser {}
impl TimeParser {
    pub fn parse(&self, text: &str) -> Result<NaiveTime, TimeParseError> {
        self.parse_relative(text, Some(&Utc::now().time()))
    }

    pub fn parse_relative(
        &self,
        text: &str,
        now: Option<&NaiveTime>,
    ) -> Result<NaiveTime, TimeParseError> {
        unimplemented!()
        // TimeExpr.recognize(text)

        // match TimeExpr
        // create NaiveTime based on DateExpr and now
    }
}

// Expressions
#[derive(Debug, PartialEq)]
/// 0 means 1 BC, -1 means 2 BC, etc.?
struct Year(pub isize);

#[derive(Debug, PartialEq)]
enum MonthOfYear {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[derive(Debug, PartialEq)]
struct Month {
    year: Year,
    month: MonthOfYear,
}

#[derive(Debug, PartialEq)]
enum DayOfWeek {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

#[derive(Debug, PartialEq)]
enum YearExpr {
    ThisYear,
    Absolute(Year),
    // OfMonth(Box<MonthExpr>),
    // OfWeek(Box<WeekExpr>),
    // OfDay(Box<DateExpr>),
    // Since(Box<YearExpr>, Duration)
    InNYears(usize),
}

#[derive(Debug, PartialEq)]
enum MonthExpr {
    ThisMonth,
    Absolute(Month),
    InYear(Box<YearExpr>, MonthOfYear),
    // OfWeek(Box<WeekExpr>),
    // OfDay(Box<DateExpr>),
    // Since(Box<MonthExpr>, Duration),
    // NthSince(Box<MonthExpr>, isize, MonthOfYear),
    InNMonths(usize),
}

#[derive(Debug, PartialEq)]
enum WeekExpr {
    ThisWeek,
    Absolute(Year, i8),
    InMonth(Box<MonthExpr>, i8),
    // InYear(Box<YearExpr>, i8),
    // OfDay(Box<DateExpr>),
    // Since(Box<WeekExpr>, Duration),
    InNWeeks(usize),
}

#[derive(Debug, PartialEq)]
enum DateExpr {
    Today,
    Absolute(NaiveDate),
    InWeek(Box<WeekExpr>, DayOfWeek),
    // InMonth(Box<MonthExpr>, i8),
    // InYear(Box<YearExpr>, i16),
    // Since(Box<DateExpr>, Duration),
    // NthSince(Box<DateExpr>, isize, DayOfWeek)
    InNDays(usize),
}

#[derive(Debug, PartialEq)]
enum TimeExpr {
    Now,
    Absolute(NaiveTime),
    InDay(Box<DateExpr>),
    InNHours(usize),
    InNMins(usize),
}

// Recognize

// /// An approximate parsing result:
// struct Recognized<'a, T> {
//     value: T,
//     confidence: f32,
//     rest: &'a str,
// }

/// Trait for types that can be parsed.
trait Recognizable: Sized {
    type Error: std::error::Error;

    fn recognize(text: &str) -> Result<Self, Self::Error>;

    fn describe() -> &'static str;
}

impl Recognizable for YearExpr {
    type Error = DateParseError;

    fn recognize(text: &str) -> Result<YearExpr, Self::Error> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "month"
    }
}

impl Recognizable for MonthExpr {
    type Error = DateParseError;

    fn recognize(text: &str) -> Result<MonthExpr, Self::Error> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "month"
    }
}

impl Recognizable for WeekExpr {
    type Error = DateParseError;

    fn recognize(text: &str) -> Result<WeekExpr, Self::Error> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "week"
    }
}

impl Recognizable for DateExpr {
    type Error = DateParseError;

    fn recognize(text: &str) -> Result<DateExpr, Self::Error> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "date"
    }
}

// https://github.com/wanasit/chrono/blob/master/src/parsers/en/ENTimeExpressionParser.js
impl Recognizable for TimeExpr {
    type Error = TimeParseError;

    fn recognize(text: &str) -> Result<TimeExpr, Self::Error> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "time of day"
    }
}

// Tests
#[cfg(test)]
mod time_expr_tests {
    use super::{Recognizable, TimeExpr};
    use chrono::NaiveTime;

    #[test]
    fn simple_hour_tests() {
        assert_recognize_time("12", 12, 0);
    }

    #[test]
    fn am_pm_hour_tests() {
        assert_recognize_time("10am", 10, 0);
        assert_recognize_time("10pm", 22, 0);
    }

    #[test]
    fn simple_minute_tests() {
        assert_recognize_time("12:30", 12, 30);
    }

    #[test]
    fn am_pm_minute_tests() {
        assert_recognize_time("10:30am", 10, 30);
        assert_recognize_time("10pm", 22, 30);
    }

    fn assert_recognize_time(text: &str, expected_h: u32, expected_m: u32) {
        // assert_eq!(
        //     TimeExpr::recognize(text),
        //     Ok((
        //         TimeExpr::Absolute(NaiveTime::from_hms(expected_h, expected_m, 0)),
        //         text
        //     ))
        // )
    }
}

// Fuzzy Helpers?
