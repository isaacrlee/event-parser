use chrono::{Datelike, NaiveDate, Utc};
use regex::Regex;
use std::error::Error;
use std::fmt;

use crate::recognizable::Recognizable;

#[derive(Debug, PartialEq)]
/// The error type for date parsing.
pub enum DateParseError {
    DateUnknown,
    DateBad, // E.g. January 45th
}

impl fmt::Display for DateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateParseError::DateUnknown => write!(f, "Error: Date unknown"),
            DateParseError::DateBad => write!(f, "Error: Bad date"),
        }
    }
}

impl Error for DateParseError {
    fn description(&self) -> &str {
        "Date unknown"
    }
}

/// A date parser for string slices.
pub struct DateParser {}

impl DateParser {
    pub fn parse(text: &str) -> Result<Option<NaiveDate>, DateParseError> {
        DateParser::parse_relative(text, &Utc::now().date().naive_utc())
    }

    pub fn parse_relative(
        text: &str,
        now: &NaiveDate,
    ) -> Result<Option<NaiveDate>, DateParseError> {
        //unimplemented!()
        let date_opt = DateExpr::recognize(text)?;

        match date_opt {
            Some(expr) => match expr {
                DateExpr::InMonth(m, d) => {
                    let nd = NaiveDate::from_ymd(now.year(), m as u32, d);
                    return Ok(Some(nd));
                }
                DateExpr::MonthDateYear(m, d, y) => {
                    let nd = NaiveDate::from_ymd(y, m, d);
                    return Ok(Some(nd));
                }
                _ => {}
            },
            None => return Ok(None),
        }
        Ok(None)
    }
}

#[derive(Debug, PartialEq)]
/// 0 means 1 BC, -1 means 2 BC, etc.?
struct Year(pub isize);

#[derive(Debug, PartialEq)]
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

/// Convert a u32 into a MonthOfYear
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

// #[derive(Debug, PartialEq)]
// struct Month {
//     year: Year,
//     month: MonthOfYear,
// }

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

// #[derive(Debug, PartialEq)]
// enum YearExpr {
//     ThisYear,
//     Absolute(Year),
//     // OfMonth(Box<MonthExpr>),
//     // OfWeek(Box<WeekExpr>),
//     // OfDay(Box<DateExpr>),
//     // Since(Box<YearExpr>, Duration)
//     InNYears(usize),
// }

// #[derive(Debug, PartialEq)]
// enum MonthExpr {
//     ThisMonth,
//     Absolute(MonthOfYear),
//     InYear(Box<YearExpr>, MonthOfYear),
//     // OfWeek(Box<WeekExpr>),
//     // OfDay(Box<DateExpr>),
//     // Since(Box<MonthExpr>, Duration),
//     // NthSince(Box<MonthExpr>, isize, MonthOfYear),
//     InNMonths(usize),
// }

// #[derive(Debug, PartialEq)]
// enum WeekExpr {
//     ThisWeek,
//     Absolute(Year, i8),
//     InMonth(Box<MonthExpr>, i8),
//     // InYear(Box<YearExpr>, i8),
//     // OfDay(Box<DateExpr>),
//     // Since(Box<WeekExpr>, Duration),
//     InNWeeks(usize),
// }

#[derive(Debug, PartialEq)]
// An abstract syntax for parsing dates.
enum DateExpr {
    Today,
    MonthDateYear(u32, u32, i32), // TODO: Replace this with InYear
    DayInNWeeks(i8, DayOfWeek),   // e.g. next week monday => DayInNWeeks(1, Mon)
    InMonth(MonthOfYear, u32),    // e.g. June 8th => InMonth(Jun, 8)
    InYear(Year, i16),
    // Since(Box<DateExpr>, Duration),
    // NthSince(Box<DateExpr>, isize, DayOfWeek)
    InNDays(usize),
}

// impl Recognizable for YearExpr {
//     type Error = DateParseError;

//     fn recognize(text: &str) -> Result<Option<YearExpr>, Self::Error> {
//         unimplemented!()
//     }

//     fn describe() -> &'static str {
//         "month"
//     }
// }

// impl Recognizable for MonthExpr {
//     type Error = DateParseError;

//     fn recognize(text: &str) -> Result<Option<MonthExpr>, Self::Error> {
//         unimplemented!()
//     }

//     fn describe() -> &'static str {
//         "month"
//     }
// }

// impl Recognizable for WeekExpr {
//     type Error = DateParseError;

//     fn recognize(text: &str) -> Result<Option<WeekExpr>, Self::Error> {
//         unimplemented!()
//     }

//     fn describe() -> &'static str {
//         "week"
//     }
// }

impl Recognizable for DateExpr {
    type Error = DateParseError;

    fn recognize(text: &str) -> Result<Option<DateExpr>, Self::Error> {
        if let Ok(Some(date)) = parse_month_date_year(text) {
            return Ok(Some(date));
        }
        if let Ok(Some(date)) = parse_in_month(text) {
            return Ok(Some(date));
        }
        if let Ok(Some(date)) = parse_month_date_english(text) {
            return Ok(Some(date));
        }

        Ok(None)
    }

    fn describe() -> &'static str {
        "date"
    }
}

impl Recognizable for DayOfWeek {
    type Error = DateParseError;

    fn recognize(text: &str) -> Result<Option<DayOfWeek>, Self::Error> {
        unimplemented!()
    }

    fn describe() -> &'static str {
        "day of week"
    }
}

impl Recognizable for MonthOfYear {
    type Error = DateParseError;

    fn recognize(text: &str) -> Result<Option<MonthOfYear>, Self::Error> {
        parse_month_of_year_english(text)
    }

    fn describe() -> &'static str {
        "month of year"
    }
}

// Examples
// (12pm, 12, noon, twelve, at 12, 10:30, 12:30pm}
// {Saturday, 6/1, sat, this saturday, next saturday, last saturday, june 1, june 1st}
// {tonight, last night, tomorrow night, tomorrow morning, lunch, dinner, breakfast, dawn, late, afternoon, evening, now, in two hours, midnight}

/// Returns an `Option` containing a `DateExpr::Absolute(NaiveDate)`
fn parse_in_month(text: &str) -> Result<Option<DateExpr>, DateParseError> {
    // 6/1, 06/01, 06-01-15

    let re = Regex::new(r"(?P<month>\d{1,2})(/|-)(?P<date>\d{1,2})").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let month: u32 = caps["month"].parse().unwrap();
        let date: u32 = caps["date"].parse().unwrap();

        return Ok(Some(DateExpr::InMonth(num_to_month(month).unwrap(), date)));
    }

    Ok(None)
}

fn parse_month_date_year(text: &str) -> Result<Option<DateExpr>, DateParseError> {
    // 6/1, 06/01, 06-01-15

    let re =
        Regex::new(r"(?P<month>\d{1,2})(/|-)(?P<date>\d{1,2})(/|-)(?P<year>\d{4}|\d{2})").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let month: u32 = caps["month"].parse().unwrap();
        let date: u32 = caps["date"].parse().unwrap();
        let year: i32 = caps["year"].parse().unwrap();
        return Ok(Some(DateExpr::MonthDateYear(month, date, year)));
    }

    Ok(None)
}

/// Returns an `Option` containing a `DateExpr::Absolute(NaiveDate)`
fn parse_month_date_english(text: &str) -> Result<Option<DateExpr>, DateParseError> {
    //june 1, june 1st
    // Generalize for having the date before the month, not just after
    let re = Regex::new(r"(?i)(?P<month>jan|january|feb|mar|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)\s(?P<date>\d{1,2})?").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let month_str = caps["month"].to_lowercase();
        let date: u32 = caps["date"].parse().unwrap();
        if let Some(m) = MonthOfYear::recognize(&month_str)? {
            return Ok(Some(DateExpr::InMonth(m, date)));
        }
    }

    Ok(None)
}

/// Returns an `Option` containing a `DateExpr::InWeek(Box<WeekExpr>, DayOfWeek)`
fn parse_date_in_week(text: &str) -> Result<Option<DateExpr>, DateParseError> {
    // sat, this saturday, next saturday, last saturday, this sat,

    unimplemented!()
}

/// Returns an `Option` containing a `DateExpr::InNDays(usize)`
fn parse_relative_date(text: &str) -> Result<Option<DateExpr>, DateParseError> {
    // in two days, in 2 days
    unimplemented!()
}

fn parse_day_of_week(text: &str) -> Result<Option<DayOfWeek>, DateParseError> {
    unimplemented!()
}

fn parse_month_of_year_english(text: &str) -> Result<Option<MonthOfYear>, DateParseError> {
    let re = Regex::new(r"(?i)(?P<month>jan|january|feb|mar|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)").unwrap();

    if let Some(caps) = re.captures_iter(text).next() {
        let month = caps["month"].to_lowercase();
        match month.as_ref() {
            "jan" => return Ok(Some(MonthOfYear::Jan)),
            "feb" => return Ok(Some(MonthOfYear::Feb)),
            "mar" => return Ok(Some(MonthOfYear::Mar)),
            "apr" => return Ok(Some(MonthOfYear::Apr)),
            "may" => return Ok(Some(MonthOfYear::May)),
            "jun" => return Ok(Some(MonthOfYear::Jun)),
            "jul" => return Ok(Some(MonthOfYear::Jul)),
            "aug" => return Ok(Some(MonthOfYear::Aug)),
            "sep" => return Ok(Some(MonthOfYear::Sep)),
            "oct" => return Ok(Some(MonthOfYear::Oct)),
            "nov" => return Ok(Some(MonthOfYear::Nov)),
            "dec" => return Ok(Some(MonthOfYear::Dec)),
            _ => return Ok(None),
        }
    }

    Ok(None)
}

// Tests
#[cfg(test)]
mod date_expr_tests {
    use super::{
        DateExpr,
        MonthOfYear::{self, *},
        Recognizable,
    };
    //use chrono::NaiveDate;

    #[test]
    fn month_date_tests() {
        assert_recognize_in_month("06/05", Jun, 5);
        assert_recognize_in_month("06-05", Jun, 5);
        assert_recognize_in_month("6/5", Jun, 5);
        assert_recognize_in_month("6-5", Jun, 5);
        assert_recognize_in_month("6/15", Jun, 15);
        assert_recognize_in_month("12/15", Dec, 15);
        assert_recognize_in_month("12/6", Dec, 6);
        // assert_recognize_date("12/15/19", 12, 15);
    }

    #[test]
    fn month_date_year_tests() {
        assert_recognize_month_date_year("12/15/19", 12, 15, 19);
        assert_recognize_month_date_year("12/15/2000", 12, 15, 2000);
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

    // #[test]
    // fn absolute_day_tests() {
    //     assert_recognize_date("Mon", 6, 5);
    // }

    fn assert_recognize_in_month(text: &str, expected_m: MonthOfYear, expected_d: u32) {
        assert_eq!(
            DateExpr::recognize(text),
            Ok(Some(DateExpr::InMonth(expected_m, expected_d)))
        )
    }

    fn assert_recognize_month_date_year(text: &str, m: u32, d: u32, y: i32) {
        assert_eq!(
            DateExpr::recognize(text),
            Ok(Some(DateExpr::MonthDateYear(m, d, y)))
        )
    }
}

mod month_expr_tests {
    use super::{
        MonthOfYear::{self, *},
        Recognizable,
    };
    use chrono::NaiveDate;

    // #[test]
    // fn absolute_month_tests() {
    //     assert_recognize_month("06/05", MonthOfYear::Jun);
    // }

    #[test]
    fn english_month_tests() {
        assert_recognize_month("summer in June", Jun);
        assert_recognize_month("mother's day in May", May);
        assert_recognize_month("back to school in August", Aug);
        assert_recognize_month("Lunch w/Julie apr", Apr);
        assert_recognize_month("octopus 8pm jul", Jul);
        assert_recognize_month("julie 7 jul 5", Jul);
    }

    // #[test]
    // fn absolute_day_tests() {
    //     assert_recognize_date("Mon", 6, 5, 19);
    // }

    fn assert_recognize_month(text: &str, expected_m: MonthOfYear) {
        assert_eq!(MonthOfYear::recognize(text), Ok(Some(expected_m)))
    }
}
