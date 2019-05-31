use chrono::{NaiveDate, Utc};
use std::error::Error;
use std::fmt;

use crate::recognizable::Recognizable;

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
