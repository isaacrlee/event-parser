//! # Date Time Parser
//!
//! A library for parsing unstructered text into `NaiveDate` and `NaiveTime` formats.
//!
//! ### Usage and Sample Inputs
//!
//! **Times:**  
//! 12pm, 12, noon, twelve, at 12, 10:30, 12:30pm, midnight
//!
//! **Days:**  
//! Saturday, 6/1, sat, this Saturday, next saturday, last saturday, june 1, June 1st
//!
//! **Relative days and times:**  
//! tonight, last night, tomorrow night, tomorrow morning, afternoon, evening, now, in two hours

pub mod date_parse;
pub mod recognizable;
pub mod time_parse;
