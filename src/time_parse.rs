use chrono::{Duration, NaiveTime, Utc};
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
    //RegexError
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

#[derive(Default)]
/// A time parser for string slices.
pub struct TimeParser {}

impl TimeParser {
    /// Parses this string slice into an option containing a `NaiveTime`.
    /// # Example
    /// ```
    /// use chrono::NaiveTime;
    /// use eventparser::{time_parse::TimeParser, recognizable::Recognizable};
    ///
    /// let time = TimeParser::parse("6:30pm");
    /// assert_eq!(time, Ok(Some((NaiveTime::from_hms(18, 30, 0)))));
    /// ```
    pub fn parse(text: &str) -> Result<Option<NaiveTime>, TimeParseError> {
        TimeParser::parse_relative(text, &Utc::now().time())
    }

    /// Parses this string slice into an option containing a `NaiveTime` relative to `now`.
    /// # Example
    /// ```
    /// use chrono::{NaiveTime, Utc};
    /// use eventparser::{time_parse::TimeParser, recognizable::Recognizable};
    /// let time = TimeParser::parse_relative("6:30pm", &Utc::now().time());
    /// assert_eq!(time, Ok(Some((NaiveTime::from_hms(18, 30, 0)))));
    pub fn parse_relative(
        text: &str,
        now: &NaiveTime,
    ) -> Result<Option<NaiveTime>, TimeParseError> {
        let time_opt = TimeExpr::recognize(text)?;

        match time_opt {
            Some(expr) => match expr {
                TimeExpr::Absolute(nt) => {
                    return Ok(Some(nt));
                }
                TimeExpr::InNHours(h) => {
                    let d = Duration::hours(h as i64);
                    return Ok(Some(now.overflowing_add_signed(d).0));
                }
                TimeExpr::InNMins(m) => {
                    let d = Duration::minutes(m as i64);
                    return Ok(Some(now.overflowing_add_signed(d).0));
                }
                _ => {}
            },
            None => return Ok(None),
        }
        Ok(None)
    }
}

#[derive(Debug, PartialEq)]
// An abstract syntax for parsing times.
enum TimeExpr {
    Now,
    Absolute(NaiveTime),
    InNHours(u32),
    InNMins(u32),
}

// https://github.com/wanasit/chrono/blob/master/src/parsers/en/ENTimeExpressionParser.js
impl Recognizable for TimeExpr {
    type Error = TimeParseError;

    fn recognize(text: &str) -> Result<Option<TimeExpr>, Self::Error> {
        if let Ok(Some(time)) = parse_relative_time(text) {
            return Ok(Some(time));
        }
        if let Ok(Some(time)) = parse_absolute_time(text) {
            return Ok(Some(time));
        }
        if let Ok(Some(time)) = parse_casual_time(text) {
            return Ok(Some(time));
        }
        Ok(None)
    }

    fn describe() -> &'static str {
        "time of day"
    }
}

fn parse_absolute_time(text: &str) -> Result<Option<TimeExpr>, TimeParseError> {
    let re =
        Regex::new(r"(?i)(^|\b)(?P<hour>\d{1,2}):?(?P<minute>\d{2})?(?P<meridiem>[ap]m?)?($|\b)")
            .unwrap();

    if let Some(caps) = re.captures(text) {
        let mut hour: u32 = 0;
        let mut minute = 0;

        if let Some(hour_match) = caps.name("hour") {
            hour = hour_match.as_str().parse().unwrap();
        }

        // contains a minute value
        if let Some(minute_match) = caps.name("minute") {
            minute = minute_match.as_str().parse().unwrap();
        }

        // contains am or pm
        if let Some(meridiem_match) = caps.name("meridiem") {
            if meridiem_match.as_str().to_lowercase().contains('p') && hour != 12 {
                hour += 12;
            } else {

            }
        } else {
            // doesn't contain am or pm, default is pm for 1-8 and am for 9-12
            if hour < 9 {
                hour += 12;
            }
        }

        return Ok(Some(TimeExpr::Absolute(NaiveTime::from_hms(
            hour, minute, 0,
        ))));
    }

    Ok(None)
}

fn parse_casual_time(text: &str) -> Result<Option<TimeExpr>, TimeParseError> {
    // "morning", "evening", "midnight", "mid{-}?day", ...?

    let casual_phrases = vec![
        r"morning",
        r"afternoon",
        r"evening",
        r"tonight",
        r"noon",
        r"midnight",
    ];
    let hours = vec![9, 14, 18, 21, 12, 0];

    for (i, phrase) in casual_phrases.iter().enumerate() {
        let re = Regex::new(phrase).unwrap();
        // println!("match: {:?}", re.find(&text));
        if let Some(time) = re.find(&text) {
            // println!("hour: {}", hours[i]);
            return Ok(Some(TimeExpr::Absolute(NaiveTime::from_hms(
                hours[i], 0, 0,
            ))));
        }
    }

    Ok(None)
}

fn parse_relative_time(text: &str) -> Result<Option<TimeExpr>, TimeParseError> {
    // "in_hours/minutes",
    let re = Regex::new(r"in (?P<mins>\d{1,2}) (mins|minutes|min|minute)").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let mut mins: u32 = caps["mins"].parse().unwrap();
        return Ok(Some(TimeExpr::InNMins(mins)));
    }

    let re = Regex::new(r"in (?P<hours>\d{1,2}) (hrs|hours|hr|hour)").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let mut hours: u32 = caps["hours"].parse().unwrap();
        return Ok(Some(TimeExpr::InNHours(hours)));
    }

    Ok(None)
}

// Tests
#[cfg(test)]
mod time_expr_tests {
    use super::{Recognizable, TimeExpr};
    use chrono::NaiveTime;

    #[test]
    fn simple_hour_tests() {
        assert_recognize_time("12", 12, 0);
        assert_recognize_time("2", 14, 0);
        assert_recognize_time("10", 10, 0);
        assert_recognize_time("5", 17, 0);
        assert_recognize_time("at 5", 17, 0);
    }

    #[test]
    fn am_pm_hour_tests() {
        assert_recognize_time("10am", 10, 0);
        assert_recognize_time("10pm", 22, 0);
        assert_recognize_time("12pm", 12, 0);
        assert_recognize_time("2p", 14, 0);
    }

    #[test]
    fn simple_minute_tests() {
        assert_recognize_time("12:30", 12, 30);
        assert_recognize_time("2:30", 14, 30);
    }

    #[test]
    fn am_pm_minute_tests() {
        assert_recognize_time("10:30am", 10, 30);
        assert_recognize_time("2:30pm", 14, 30);
        assert_recognize_time("10:30AM", 10, 30);
        assert_recognize_time("2:30PM", 14, 30);
        assert_recognize_time("10:30a", 10, 30);
        assert_recognize_time("2:30p", 14, 30);
    }

    #[test]
    fn casual_time_tests() {
        assert_recognize_time("in the morning", 9, 0);
        assert_recognize_time("this afternoon", 14, 0);
        assert_recognize_time("in the evening", 18, 0);
        assert_recognize_time("tonight", 21, 0);
        assert_recognize_time("noon", 12, 0);
        assert_recognize_time("midnight", 0, 0);
    }

    #[test]
    fn relative_mins_time_tests() {
        assert_in_mins_time("in 5 mins", 5);
        assert_in_mins_time("in 10 minutes", 10);
        assert_in_mins_time("in 1 min", 1);
    }

    #[test]
    fn relative_hours_time_tests() {
        assert_in_hours_time("in 2 hours", 2);
        assert_in_hours_time("in 3 hrs", 3);
        assert_in_hours_time("in 1 hr", 1);
        assert_in_hours_time("in 1 hour", 1);
    }

    fn assert_recognize_time(text: &str, expected_h: u32, expected_m: u32) {
        assert_eq!(
            TimeExpr::recognize(text),
            Ok(Some(TimeExpr::Absolute(NaiveTime::from_hms(
                expected_h, expected_m, 0
            ))))
        )
    }

    fn assert_in_mins_time(text: &str, expected_m: u32) {
        assert_eq!(
            TimeExpr::recognize(text),
            Ok(Some(TimeExpr::InNMins(expected_m)))
        )
    }

    fn assert_in_hours_time(text: &str, expected_m: u32) {
        assert_eq!(
            TimeExpr::recognize(text),
            Ok(Some(TimeExpr::InNHours(expected_m)))
        )
    }
}
