//! # Date Time Parser
//!
//! A library for parsing structured and unstructered text into `NaiveDate` and `NaiveTime` formats."

pub mod date_parse;
pub mod recognizable;
pub mod time_parse;

// Examples
// (12pm, 12, noon, twelve, at 12, 10:30, 12:30pm}
// {Saturday, 6/1, sat, this saturday, next saturday, last saturday, june 1, june 1st}
// {tonight, last night, tomorrow night, tomorrow morning, lunch, dinner, breakfast, dawn, late, afternoon, evening, now, in two hours, midnight}
