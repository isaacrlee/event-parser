use chrono::{NaiveTime, Utc};
use regex::*;
use std::error::Error;
use std::fmt;

// use crate::date_parse::*;
use crate::recognizable::Recognizable;

extern crate regex;

#[derive(Debug, PartialEq)]
pub enum TimeParseError {
    TimeUnknown,
    TimeBad,
}

impl fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeParseError::TimeUnknown => write!(f, "Error: Time unknown"),
            TimeParseError::TimeBad => write!(f, "Error: Time bad format"),
        }
    }
}

impl Error for TimeParseError {
    fn description(&self) -> &str {
        "Time unknown"
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

#[derive(Debug, PartialEq)]
enum TimeExpr {
    Now,
    Absolute(NaiveTime),
    // InDay(Box<DateExpr>),
    InNHours(usize),
    InNMins(usize),
}

// https://github.com/wanasit/chrono/blob/master/src/parsers/en/ENTimeExpressionParser.js
impl Recognizable for TimeExpr {
    type Error = TimeParseError;

    fn recognize(text: &str) -> Result<TimeExpr, Self::Error> {
        match try_absolute_time(text) {
            Some(expr) => Ok(expr),
            None => Err(TimeParseError::TimeUnknown),
        }
    }

    fn describe() -> &'static str {
        "time of day"
    }
}

fn try_absolute_time(text: &str) -> Option<TimeExpr> {
    // colon, "am", "pm", "o'clock", ...?

    let mut nt = NaiveTime::from_hms(0, 0, 0);

    // 8:30am/pm AM/PM/ a/p
    let re = Regex::new(r"(?i)\d{1,2}:\d{2}[ap]m?").unwrap();

    match re.find(text) {
        Some(m) => {
            println!("m: {:?}", m.as_str());
            match NaiveTime::parse_from_str(m.as_str(), "%k:%M%P") {
                Ok(t) => {
                    println!("t: {}", t);
                    nt = t;
                }
                Err(e) => {}
            }
        }
        None => {}
    }

    // 8:30, 10:30
    // let re = Regex::new(r"\d{1,2}:\d{2}").unwrap();
    // hour = re.captures_iter(text).next()

    // 10pm

    // 10
    Some(TimeExpr::Absolute(nt))
}

fn try_casual_time(text: &str) -> Option<TimeExpr> {
    // "morning", "evening", "midnight", "mid{-}?day", ...?
    None
}

fn try_relative_time(text: &str) -> Option<TimeExpr> {
    // "in_hours/minutes",
    None
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
        // assert_recognize_time("10pm", 22, 30);
    }

    fn assert_recognize_time(text: &str, expected_h: u32, expected_m: u32) {
        assert_eq!(
            TimeExpr::recognize(text),
            Ok(TimeExpr::Absolute(NaiveTime::from_hms(
                expected_h, expected_m, 0
            )))
        )
    }
}
