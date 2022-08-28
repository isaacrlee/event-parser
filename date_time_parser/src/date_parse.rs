//! Parse natural language text into the [`NaiveDate`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveDate.html) format.

use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use regex::Regex;

use crate::recognizable::Recognizable;

/// Container for parsing dates from string slices.  
pub struct DateParser {}

impl DateParser {
    /// Parses a string slice of natural language text with respect to the current date. Returns a [`NaiveDate`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveDate.html) if a match is found, `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the the text to be parsed
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{prelude::*, Duration, Local, NaiveDate, NaiveDateTime, Weekday};
    /// use date_time_parser::{DateParser, Recognizable};
    ///
    /// let year = Local::now().year();
    /// let date = DateParser::parse("July 4 2020");
    ///
    /// assert_eq!(date, Some(NaiveDate::from_ymd(year, 7, 4)));
    /// ```
    pub fn parse(text: &str) -> Option<NaiveDate> {
        DateParser::parse_relative(text, Utc::now().date().naive_utc())
    }

    /// Parses a string slice of natural language text with respect to a given date. Returns a [`NaiveDate`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveDate.html) if a match is found, `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the the text to be parsed
    /// * `now` - A [`NaiveDate`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveDate.html) to interpret the natural language date around
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{prelude::*, Duration, Local, NaiveDate, NaiveDateTime, Weekday};
    /// use date_time_parser::{DateParser, Recognizable};
    ///
    /// let year = Local::now().year();
    /// let date = DateParser::parse_relative("July 4", Utc::now().date().naive_utc());
    ///
    /// assert_eq!(date, Some(NaiveDate::from_ymd(year, 7, 4)));
    /// ```
    pub fn parse_relative(text: &str, now: NaiveDate) -> Option<NaiveDate> {
        if let Some(date_expr) = DateExpr::recognize(text) {
            match date_expr {
                DateExpr::InMonth(m, d) => {
                    let nd = NaiveDate::from_ymd(now.year(), m as u32, d);
                    return Some(nd);
                }
                DateExpr::InYear(m, d, y) => {
                    let nd = NaiveDate::from_ymd(y, m as u32, d);
                    return Some(nd);
                }
                DateExpr::InNDays(n) => {
                    let d = Duration::days(n as i64);
                    return Some(now.checked_add_signed(d).unwrap());
                }
                DateExpr::DayInNWeeks(n, d) => {
                    let mut difference: i32 = (d.num_days_from_sunday() as i32)
                        - (now.weekday().num_days_from_sunday() as i32);
                    if difference < 0 {
                        difference += 7;
                    }
                    difference += 7 * (n as i32);
                    let dur = Duration::days(difference as i64);
                    return Some(now.checked_add_signed(dur).unwrap());
                }
                DateExpr::InNMonths(n) => {
                    let now_month = now.month();
                    let to_month = (now_month as i32) + n;
                    return Some(NaiveDate::from_ymd(now.year(), to_month as u32, now.day()));
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
/// A year as defined by the Gregorian calendar i.e. AD 1 = Year(1).
struct Year(pub isize);

#[derive(Debug, PartialEq)]
/// The month of the year.
enum MonthOfYear {
    Jan = 1,
    Feb = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sep = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

/// Converts the given `u32` to a `MonthOfYear`.
fn num_to_month(num: u32) -> Option<MonthOfYear> {
    match num {
        1 => Some(MonthOfYear::Jan),
        2 => Some(MonthOfYear::Feb),
        3 => Some(MonthOfYear::Mar),
        4 => Some(MonthOfYear::Apr),
        5 => Some(MonthOfYear::May),
        6 => Some(MonthOfYear::Jun),
        7 => Some(MonthOfYear::Jul),
        8 => Some(MonthOfYear::Aug),
        9 => Some(MonthOfYear::Sep),
        10 => Some(MonthOfYear::Oct),
        11 => Some(MonthOfYear::Nov),
        12 => Some(MonthOfYear::Dec),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
/// An abstract syntax for parsing dates.
enum DateExpr {
    InNDays(i32),
    DayInNWeeks(i8, Weekday), // e.g. next week monday => DayInNWeeks(1, Mon)
    InNMonths(i32),           // e.g. in 2 months => InNMonths(2)
    InMonth(MonthOfYear, u32), // e.g. June 8th => InMonth(Jun, 8)
    InYear(MonthOfYear, u32, i32), // e.g. June 8th, 2019 => InYear(Jun, 8, 2019)
}

/// Parsing a `str` into a DateExpr uses both structured formats and common phrases.
impl Recognizable for DateExpr {
    fn recognize(text: &str) -> Option<DateExpr> {
        if let Some(date) = parse_keywords(text) {
            return Some(date);
        }
        if let Some(date) = parse_relative_date(text) {
            return Some(date);
        }
        if let Some(date) = parse_in_year(text) {
            return Some(date);
        }
        if let Some(date) = parse_in_month(text) {
            return Some(date);
        }
        if let Some(date) = parse_month_date_english(text) {
            return Some(date);
        }
        if let Some(date) = parse_date_in_week(text) {
            return Some(date);
        }
        if let Some(date) = parse_in_n_months(text) {
            return Some(date);
        }
        if let Some(date) = parse_relative_month(text) {
            return Some(date);
        }
        if let Some(date) = parse_day_alone(text) {
            return Some(date);
        }

        None
    }

    fn describe() -> &'static str {
        "date"
    }
}

/// Parsing a str into a `Weekday` uses the format %W.
impl Recognizable for Weekday {
    fn recognize(text: &str) -> Option<Weekday> {
        text.parse::<Weekday>().ok()
    }

    fn describe() -> &'static str {
        "day of week"
    }
}

/// Parsing a str into a `MonthOfYear` uses english abbreviations and full names.
impl Recognizable for MonthOfYear {
    fn recognize(text: &str) -> Option<MonthOfYear> {
        parse_month_of_year_english(text)
    }

    fn describe() -> &'static str {
        "month of year"
    }
}

/// Parses common keywords into an `Option` containing a `DateExpr::InNDays(i32)`.
fn parse_keywords(text: &str) -> Option<DateExpr> {
    // today, tomorrow, yesterday

    let re = Regex::new(r"(?i)\b(?P<key>today|tomorrow|yesterday)\b").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(key_match) = caps.name("key") {
            let n = match key_match.as_str().to_lowercase().as_ref() {
                "today" => 0,
                "tomorrow" => 1,
                "yesterday" => -1,
                _ => 0,
            };
            return Some(DateExpr::InNDays(n));
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InMonth(MonthOfYear, u32)`.
fn parse_in_month(text: &str) -> Option<DateExpr> {
    // 6/1, 06/01, 06-01-15

    let re = Regex::new(r"(?P<month>\d{1,2})(/)(?P<date>\d{1,2})").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(month_match) = caps.name("month") {
            if let Some(date_match) = caps.name("date") {
                let month: u32 = month_match.as_str().parse().unwrap();
                let date: u32 = date_match.as_str().parse().unwrap();
                return Some(DateExpr::InMonth(num_to_month(month).unwrap(), date));
            }
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InYear(MonthOfYear, u32, i32)`.
fn parse_in_year(text: &str) -> Option<DateExpr> {
    // 6/1, 06/01, 06-01-15

    let re = Regex::new(r"(?P<month>\d{1,2})(/)(?P<date>\d{1,2})(/)(?P<year>\d{4}|\d{2})").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(month_match) = caps.name("month") {
            if let Some(date_match) = caps.name("date") {
                if let Some(year_match) = caps.name("year") {
                    let month: u32 = month_match.as_str().parse().unwrap();
                    let date: u32 = date_match.as_str().parse().unwrap();
                    let year: i32 = year_match.as_str().parse().unwrap();
                    return Some(DateExpr::InYear(num_to_month(month).unwrap(), date, year));
                }
            }
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InMonth(MonthOfYear, u32)`.
fn parse_month_date_english(text: &str) -> Option<DateExpr> {
    //june 1, june 1st

    // TODO: Generalize for having the date before the month, not just after
    let re = Regex::new(r"(?i)(?P<month>jan|january|feb|mar|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)\s(?P<date>\d{1,2})?").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(month_match) = caps.name("month") {
            if let Some(date_match) = caps.name("date") {
                let date: u32 = date_match.as_str().parse().unwrap();
                let month = month_match.as_str();
                if let Some(m) = MonthOfYear::recognize(month) {
                    return Some(DateExpr::InMonth(m, date));
                }
            }
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InWeek(i8, Weekday)`
fn parse_date_in_week(text: &str) -> Option<DateExpr> {
    // sat, this saturday, next saturday, last saturday, this sat,

    let re = Regex::new(r"(?i)(?P<prep>next|last|this)\s(?P<day>\w+)").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(prep_match) = caps.name("prep") {
            let p = match prep_match.as_str().to_lowercase().as_ref() {
                "next" => 1,
                "last" => -1,
                "this" => 0,
                _ => 0,
            };

            if let Some(day_match) = caps.name("day") {
                let day_str = day_match.as_str();

                if let Some(d) = Weekday::recognize(day_str) {
                    return Some(DateExpr::DayInNWeeks(p, d));
                }
            }
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InWeek(i8, Weekday)`
fn parse_day_alone(text: &str) -> Option<DateExpr> {
    // saturday

    let re = Regex::new(r"(?i)(?P<day>mon|tue|wed|thu|fri|sat|sun)(r?day|r?sday|nesay|urday)?\b")
        .unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(day_match) = caps.name("day") {
            let d = day_match
                .as_str()
                .to_lowercase()
                .parse::<Weekday>()
                .unwrap();
            return Some(DateExpr::DayInNWeeks(0, d));
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InNDays(i32)`
fn parse_relative_date(text: &str) -> Option<DateExpr> {
    // in two days, in 2 days

    let re = Regex::new(r"(in\s(?P<num>\d{1,3})\s(days?))").unwrap();
    if let Some(caps) = re.captures(text) {
        if let Some(num_match) = caps.name("num") {
            let num: i32 = num_match.as_str().parse().unwrap();
            return Some(DateExpr::InNDays(num));
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InNMonths(i32)`
fn parse_relative_month(text: &str) -> Option<DateExpr> {
    // this month, next month, last month
    let re = Regex::new(r"(?i)(?P<prep>next|last|this)\smonth").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(prep_match) = caps.name("prep") {
            let p = match prep_match.as_str().to_lowercase().as_ref() {
                "next" => 1,
                "last" => -1,
                "this" => 0,
                _ => 0,
            };

            return Some(DateExpr::InNMonths(p));
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `DateExpr::InNMonths(i32)`
fn parse_in_n_months(text: &str) -> Option<DateExpr> {
    // in 2 months

    let re = Regex::new(r"(in\s(?P<num>\d{1,3})\s(months?))").unwrap();
    if let Some(caps) = re.captures(text) {
        if let Some(num_match) = caps.name("num") {
            let num: i32 = num_match.as_str().parse().unwrap();
            return Some(DateExpr::InNMonths(num));
        }
    }

    None
}

/// Parses a `str` into an `Option` containing a `MonthOfYear`.
fn parse_month_of_year_english(text: &str) -> Option<MonthOfYear> {
    let re = Regex::new(r"(?i)(?P<month>jan|january|feb|mar|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(month_match) = caps.name("month") {
            match month_match.as_str().to_lowercase().as_ref() {
                "jan" => return Some(MonthOfYear::Jan),
                "feb" => return Some(MonthOfYear::Feb),
                "mar" => return Some(MonthOfYear::Mar),
                "apr" => return Some(MonthOfYear::Apr),
                "may" => return Some(MonthOfYear::May),
                "jun" => return Some(MonthOfYear::Jun),
                "jul" => return Some(MonthOfYear::Jul),
                "aug" => return Some(MonthOfYear::Aug),
                "sep" => return Some(MonthOfYear::Sep),
                "oct" => return Some(MonthOfYear::Oct),
                "nov" => return Some(MonthOfYear::Nov),
                "dec" => return Some(MonthOfYear::Dec),
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod date_expr_tests {
    use super::{
        num_to_month, DateExpr,
        MonthOfYear::{self, *},
        Recognizable,
    };

    use chrono::Weekday::{self, *};

    #[test]
    fn in_month_tests() {
        assert_recognize_in_month("06/05", Jun, 5);
        assert_recognize_in_month("6/5", Jun, 5);
        assert_recognize_in_month("6/15", Jun, 15);
        assert_recognize_in_month("12/15", Dec, 15);
        assert_recognize_in_month("12/6", Dec, 6);
        // assert_recognize_date("12/15/19", 12, 15);
    }

    #[test]
    fn in_year_tests() {
        assert_recognize_in_year("12/15/19", 12, 15, 19);
        assert_recognize_in_year("12/15/2000", 12, 15, 2000);
    }

    #[test]
    fn absolute_english_date_tests() {
        assert_recognize_in_month("Jun 15", Jun, 15);
        assert_recognize_in_month("June 5th", Jun, 5);
        assert_recognize_in_month("June 5", Jun, 5);

        assert_recognize_in_month("Jan 15", Jan, 15);
        assert_recognize_in_month("February 5th", Feb, 5);
        assert_recognize_in_month("May 25", May, 25);
    }

    #[test]
    fn in_n_days_tests() {
        assert_in_n_days("Lunch in 6 days", 6);
        assert_in_n_days("Lunch in 1 day", 1);
        assert_in_n_days("Lunch in 300 days", 300);
        // assert_in_n_days("Lunch in six days", 6);
    }

    #[test]
    fn day_in_n_weeks() {
        assert_day_in_n_weeks("next thursday", Thu, 1);
        assert_day_in_n_weeks("last wed", Wed, -1);
        assert_day_in_n_weeks("this monday", Mon, 0);
        assert_day_in_n_weeks("next friday", Fri, 1);
    }

    #[test]
    fn day_keywords() {
        assert_in_n_days("tomorrow", 1);
        assert_in_n_days("yesterday", -1);
        assert_in_n_days("today", 0);
    }

    #[test]
    fn relative_month_tests() {
        assert_relative_month("in 4 months", 4);
        assert_relative_month("in 1 month", 1);
    }

    #[test]
    fn next_month_tests() {
        assert_relative_month("next month", 1);
        assert_relative_month("this month", 0);
    }

    fn assert_recognize_in_month(text: &str, expected_m: MonthOfYear, expected_d: u32) {
        assert_eq!(
            DateExpr::recognize(text),
            Some(DateExpr::InMonth(expected_m, expected_d))
        )
    }

    fn assert_recognize_in_year(text: &str, m: u32, d: u32, y: i32) {
        assert_eq!(
            DateExpr::recognize(text),
            Some(DateExpr::InYear(num_to_month(m).unwrap(), d, y))
        )
    }

    fn assert_in_n_days(text: &str, n: i32) {
        assert_eq!(DateExpr::recognize(text), Some(DateExpr::InNDays(n)))
    }

    fn assert_day_in_n_weeks(text: &str, d: Weekday, n: i8) {
        assert_eq!(DateExpr::recognize(text), Some(DateExpr::DayInNWeeks(n, d)))
    }

    fn assert_relative_month(text: &str, expected_n: i32) {
        assert_eq!(
            DateExpr::recognize(text),
            Some(DateExpr::InNMonths(expected_n))
        )
    }
}

mod month_of_year_tests {
    use super::{
        MonthOfYear::{self},
        Recognizable,
    };

    #[test]
    fn english_month_tests() {
        assert_recognize_month("summer in June", MonthOfYear::Jun);
        assert_recognize_month("mother's day in May", MonthOfYear::May);
        assert_recognize_month("back to school in August", MonthOfYear::Aug);
        assert_recognize_month("Lunch w/Julie apr", MonthOfYear::Apr);
        assert_recognize_month("octopus 8pm jul", MonthOfYear::Jul);
        assert_recognize_month("julie 7 jul 5", MonthOfYear::Jul);
    }

    #[allow(dead_code)]
    fn assert_recognize_month(text: &str, expected_m: MonthOfYear) {
        assert_eq!(MonthOfYear::recognize(text), Some(expected_m))
    }
}
