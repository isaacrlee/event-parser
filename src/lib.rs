use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};
use regex::RegexSet;
use std::fmt;

#[derive(Debug)]
pub enum DateParseError {
    DateBadFormat,
    IoError(std::io::Error),
}

impl fmt::Display for DateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateParseError::DateBadFormat => write!(f, "Error: Date not formatted correctly"),
            DateParseError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum TimeParseError {
    TimeBadFormat,
    IoError(std::io::Error),
}

impl fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeParseError::TimeBadFormat => write!(f, "Error: Time not formatted correctly"),
            TimeParseError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

// Examples
// (12pm, 12, noon, twelve, at 12, 10:30, 12:30pm}
// {Saturday, 6/1, sat, this saturday, next saturday, last saturday, june 1, june 1st}
// {tonight, last night, tomorrow night, tomorrow morning, lunch, dinner, breakfast, dawn, late, afternoon, evening, now, in two hours, midnight}

// Parser

// DateTime Parser

/// Contains parsing options.
// pub struct DateTimeParser {}

// impl DateTimeParser {
//     pub fn parse(&self, text: &str) -> Result<DateTime<Utc>, DateTimeParseError> {
//         self.parse_relative(text, Some(&Utc::now()))
//     }

//     pub fn parse_relative(
//         &self,
//         text: &str,
//         now: Option<&DateTime<Utc>>,
//     ) -> Result<DateTime<Utc>, DateTimeParseError> {
//         unimplemented!()
//     }
// }

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
    fn recognize(text: &str) -> Option<(Self, &str)>;

    fn describe() -> &'static str;
}

impl Recognizable for YearExpr {
    fn recognize(text: &str) -> Option<(YearExpr, &str)> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "month"
    }
}

impl Recognizable for MonthExpr {
    fn recognize(text: &str) -> Option<(MonthExpr, &str)> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "month"
    }
}

impl Recognizable for WeekExpr {
    fn recognize(text: &str) -> Option<(WeekExpr, &str)> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "week"
    }
}

impl Recognizable for DateExpr {
    fn recognize(text: &str) -> Option<(DateExpr, &str)> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "date"
    }
}

// https://github.com/wanasit/chrono/blob/master/src/parsers/en/ENTimeExpressionParser.js
impl Recognizable for TimeExpr {
    fn recognize(text: &str) -> Option<(TimeExpr, &str)> {
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

// /// Returns true if and only if the text matches the pattern.
// fn is_match_helper(text: &str, patterns: &[&str]) -> bool {
//     let re = RegexSet::new(patterns).expect("Invalid Patterns");
//     re.is_match(text)
// }
