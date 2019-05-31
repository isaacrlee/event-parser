use chrono::{NaiveTime, Utc};
use std::error::Error;
use std::fmt;

// use crate::date_parse::*;
use crate::recognizable::Recognizable;

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
