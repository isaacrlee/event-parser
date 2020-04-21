//! Parse natural language text into the [`NaiveTime`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveTime.html) format.

use chrono::{Duration, NaiveTime, Utc};
use regex::*;

use crate::recognizable::Recognizable;

extern crate regex;

#[derive(Default)]
/// Container for parsing times from string slices.  
pub struct TimeParser {}

impl TimeParser {
    /// Parses a string slice of natural language text with respect to the current time. Returns a [`NaiveTime`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveTime.html) if a match is found, `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the the text to be parsed
    ///
    /// # Example
    /// ```
    /// use chrono::NaiveTime;
    /// use date_time_parser::{TimeParser, Recognizable};
    ///
    /// let time = TimeParser::parse("6:30pm");
    /// assert_eq!(time, Some(NaiveTime::from_hms(18, 30, 0)));
    /// ```
    pub fn parse(text: &str) -> Option<NaiveTime> {
        TimeParser::parse_relative(text, Utc::now().time())
    }

    /// Parses a string slice of natural language text with respect to a given time. Returns a [`NaiveTime`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveTime.html) if a match is found, `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the the text to be parsed
    /// * `now` - A [`NaiveTime`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveTime.html) to interpret the natural language date around
    ///
    /// # Example
    /// ```
    /// use chrono::{NaiveTime, Utc};
    /// use date_time_parser::{TimeParser, Recognizable};
    ///
    /// let time = TimeParser::parse_relative("6:30pm", Utc::now().time());
    /// assert_eq!(time, Some(NaiveTime::from_hms(18, 30, 0)));
    /// ```
    pub fn parse_relative(text: &str, now: NaiveTime) -> Option<NaiveTime> {
        if let Some(time_expr) = TimeExpr::recognize(text) {
            match time_expr {
                TimeExpr::Absolute(nt) => {
                    return Some(nt);
                }
                TimeExpr::InNHours(h) => {
                    let d = Duration::hours(h as i64);
                    return Some(now.overflowing_add_signed(d).0);
                }
                TimeExpr::InNMins(m) => {
                    let d = Duration::minutes(m as i64);
                    return Some(now.overflowing_add_signed(d).0);
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
// An intermediate expression for parsing unstructured text into [`NaiveTime`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveTime.html).
enum TimeExpr {
    Absolute(NaiveTime),
    InNHours(u32),
    InNMins(u32),
}

/// Parsing a `str` into a TimeExpr uses both structured formats and common phrases.
impl Recognizable for TimeExpr {
    fn recognize(text: &str) -> Option<TimeExpr> {
        if let Some(time) = parse_relative_time(text) {
            return Some(time);
        }
        if let Some(time) = parse_absolute_time(text) {
            return Some(time);
        }
        if let Some(time) = parse_casual_time(text) {
            return Some(time);
        }
        None
    }

    fn describe() -> &'static str {
        "time of day"
    }
}

fn parse_absolute_time(text: &str) -> Option<TimeExpr> {
    let re =
        Regex::new(r"(?i)(^|\b)(?P<hour>\d{1,2}):?(?P<minute>\d{2})?(?P<meridiem>[ap]m?)?($|\b)")
            .unwrap();

    let date_pattern = Regex::new(r"\d{1,2}/\d{1,2}").unwrap();
    if let Some(caps) = re.captures(&date_pattern.replace_all(text, "")) {
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

        return Some(TimeExpr::Absolute(NaiveTime::from_hms(hour, minute, 0)));
    }

    None
}

/// Parses a `str` into an `Option` containing a `TimeExpr::Absolute(NaiveTime)`.
fn parse_casual_time(text: &str) -> Option<TimeExpr> {
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
        if re.find(&text).is_some() {
            // println!("hour: {}", hours[i]);
            return Some(TimeExpr::Absolute(NaiveTime::from_hms(hours[i], 0, 0)));
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `TimeExpr::InNHours(u32)`.
fn parse_relative_time(text: &str) -> Option<TimeExpr> {
    // "in_hours/minutes"

    let re = Regex::new(r"in (?P<mins>\d{1,2}) (mins|minutes|min|minute)").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let mins: u32 = caps["mins"].parse().unwrap();
        return Some(TimeExpr::InNMins(mins));
    }

    let re = Regex::new(r"in (?P<hours>\d{1,2}) (hrs|hours|hr|hour)").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let hours: u32 = caps["hours"].parse().unwrap();
        return Some(TimeExpr::InNHours(hours));
    }

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
            Some(TimeExpr::Absolute(NaiveTime::from_hms(
                expected_h, expected_m, 0
            )))
        )
    }

    fn assert_in_mins_time(text: &str, expected_m: u32) {
        assert_eq!(
            TimeExpr::recognize(text),
            Some(TimeExpr::InNMins(expected_m))
        )
    }

    fn assert_in_hours_time(text: &str, expected_m: u32) {
        assert_eq!(
            TimeExpr::recognize(text),
            Some(TimeExpr::InNHours(expected_m))
        )
    }
}
