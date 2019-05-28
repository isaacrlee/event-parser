extern crate chrono;
extern crate icalendar;
extern crate regex;

use chrono::{NaiveDate, NaiveTime};
use icalendar::{Component, Event};
// use regex::Regex;
use std::iter;

/// Parses input string into Event
pub fn parse_input(input: &str) -> Event {
    println!("Input: {}", input);

    let mut e = Event::new();

    // summary
    if let Some(s) = get_summary(input) {
        e.summary(&s);
    }

    // location
    if let Some(loc) = get_location(input) {
        e.location(&loc);
    }

    // start time/date and end time/date
    set_start_and_end(&mut e);

    e.done()
}

/// Sets the start time/date and end time/date for an `Event`.
fn set_start_and_end(e: &mut Event) {
    // Time Parsing:
    // Looks for a start time and end time
    //  if no start time exists, then assume all day/days event
    //      look for start date and end date
    //          if no start date exists, default to today -> all-day event
    //          if start date exists abut no end date, default to 1 day -> all day event
    //          else start date and end date event
    // if start time exists but no end time, default to 1 hour

    // Assume NOT all day event: look for date
    //  if no date exists, default to today
}

/// Returns an `Iterator` of all dates found in `input` as `NaiveDate`s.
fn get_dates(input: &str) -> impl Iterator<Item = NaiveDate> {
    iter::empty::<NaiveDate>()
}

/// Returns an `Iterator` of all times found in `input` as `NaiveTime`s.
pub fn get_times(input: &str) -> impl Iterator<Item = NaiveTime> {
    iter::empty::<NaiveTime>()
    // let re = Regex::new(r"(\d+:\d+)").unwrap();
    // let mat = re.find(input);

    // if let Some(s) = mat {
    //     let mut iter = s.as_str().split(':');
    //     let h = iter.next().unwrap().parse().unwrap();
    //     let m = iter.next().unwrap().parse().unwrap();
    //     return Some(NaiveTime::from_hms(h, m, 0));
    // }
}

/// Returns an `Option` containing an event's summary string parsed from `input`.
fn get_summary(input: &str) -> Option<String> {
    Some("Example Summary".to_owned())
}

/// Returns an `Option` containing an event location string parsed from `input`.
fn get_location(input: &str) -> Option<String> {
    Some("Example Location".to_owned())
}

mod test {
    // use super::get_times;
    // use chrono::NaiveTime;

    // #[test]
    // fn colon_time_test() {
    //     assert_time("Breakfast on Friday at 10:30", 10, 30);
    // }

    // #[test]
    // fn am_pm_time_test() {
    //     assert_time("Lunch on Friday at 12pm", 12, 0);
    // }

    // fn assert_time(input: &str, expected_h: u32, expected_m: u32) {
    //     assert_eq!(
    //         get_times(&input),
    //         Some(NaiveTime::from_hms(expected_h, expected_m, 0))
    //     )
    // }
}
