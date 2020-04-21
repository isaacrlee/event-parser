//! # Date Time Parser: Rust NLP Library
//!
//! It aims to parse unstructered text into [`NaiveDate`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveDate.html) and [`NaiveTime`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveTime.html) formats.
//! * Date Time Parser has the ability to be timezone aware, but defaults to UTC.
//! * Allows for parse to be relative to current date/time, or relative to a custom date/time.
//! * Leverages [chrono](https://docs.rs/chrono/0.4.11/chrono/) and [regex](https://docs.rs/regex/1.3.6/regex/) crates.
//!
//! ## Usage
//! Put this in your `Cargo.toml`:
//! ```toml,ignore
//! [dependencies]
//! date_time_parser = "0.1.0"
//! ```
//! Then put this in your crate root:
//! ```
//! extern crate date_time_parser;
//! ```
//!
//! ## Example: Find a Date
//! General use of this package involves passing English natural language that includes a date
//! to the [`DateParser`](../date_time_parser/date_parse/struct.DateParser.html) struct to parse the expression. If a date is found, it will parse the expression into the
//! [`NaiveDate`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveDate.html) format.
//! ```
//! use date_time_parser::DateParser;
//! use chrono::NaiveDate;
//!
//! let date = DateParser::parse("Lunch on June 5th");
//! assert_eq!(date, Some(NaiveDate::from_ymd(2020, 6, 5)));
//! ```
//!
//! ## Example: Find a Time
//! Similarly to parsing dates, this package can also be used to parse time expressions. Pass
//! English natural language that includes a time (relative or absolute) to the [`TimeParser`](../date_time_parser/time_parse/struct.TimeParser.html) struct
//! to parse the text. A successful parse results in a [`NaiveTime`](https://docs.rs/chrono/0.4.0/chrono/naive/struct.NaiveTime.html) from the natural language expression.
//! ```
//! use date_time_parser::TimeParser;
//! use chrono::NaiveTime;
//!
//! let time = TimeParser::parse("6:30pm dinner");
//! assert_eq!(time, Some(NaiveTime::from_hms(18, 30, 0)));
//! ```
//!
//! ## Example: Not a Date/Time
//! If the package gets an expression that for it cannot find a valid date or time, the [`TimeParser::parse`](../date_time_parser/time_parse/struct.TimeParser.html#method.parse) function
//! will return `None` for that string.
//! ```
//! use date_time_parser::TimeParser;
//! use chrono::NaiveTime;
//!
//! let time = TimeParser::parse("foo bar");
//! assert_eq!(time, None);
//! ```

mod date_parse;
mod time_parse;
mod recognizable;
pub use date_parse::DateParser;
pub use recognizable::Recognizable;
pub use time_parse::TimeParser;
