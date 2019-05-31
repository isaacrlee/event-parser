// use std::io;
// use std::io::prelude::*;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use icalendar::{Component, Event};
// use regex::Regex;

fn main() {
    println!("Hello World");
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     parse_input(line.unwrap());
    // }
}

// Examples
// Starts: Lunch (12pm, 12, noon, twelve, at 12}
// AllDay: Dillo Day {Saturday, 6/1, sat, this saturday, next saturday, june 1, june 1st}
// StartsAndEnds: Concert {7-9pm, 7-9, 7 to 9, from 7 to 9, from seven to nine}
// AllDayStartsAndEnds: Welcome Week {9/1-9/8, September {1st-8, 1-8}}

// Structs
/// Abstract expression for the start and end of an event.
enum EventStartAndEndExpr {
    Starts(NaiveTime),
    StartsAndEnds(NaiveTime, NaiveTime),
    StartsWithDate(NaiveTime, NaiveDate),
    StartsAndEndsWithDate(NaiveTime, NaiveTime, NaiveDate),
    AllDay(NaiveDate),
    AllDayStartsAndEnds(NaiveDate, NaiveDate),
}

// Parse Function

/// Parses input string into Event
pub fn parse_input(text: &str) -> Event {
    println!("Input: {}", text);

    let mut e = Event::new();

    let now = chrono::Local::now();
    let today = chrono::Local::today();

    // start time/date and end time/date
    if let Some(exp) = get_start_and_end(text) {
        use EventStartAndEndExpr::*;
        match exp {
            Starts(t) => {
                // check if parsed time is later than current time, if so, set default day to tomorrow, otherwise, default to today

                let ndt = NaiveDateTime::new(today.naive_utc(), t);
                // e.starts(DateTime::<Local>::from_utc(ndt, Local));
            } // ...
            _ => {}
        }
    }

    // location
    if let Some(loc) = get_location(text) {
        e.location(&loc);
    }

    // summary
    if let Some(summary) = get_summary(text) {
        e.summary(&summary);
    }

    e.done()
}

/// Returns an `Option` containing an `EventStartAndEndExpr`.
fn get_start_and_end(input: &str) -> Option<EventStartAndEndExpr> {
    unimplemented!()
    // Hack: look for {'-', "to"}, if found, then it's a StartsAndEnds, StartsAndEndsWithDate, or AllDayStartsAndEnds
    //  Get expressions before and after {'-', "to"}
    // else parse input for Start
    //  Hack: search for {12am, midnight}, if not found, if 12am is returned, it's an AllDay, or AllDayStartsAndEnds
    //  else it's a Starts
    // if not found, default to now

    // Previous Time Parsing:
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

/// Returns an `Option` containing an event's summary string parsed from `input`.
fn get_summary(input: &str) -> Option<String> {
    Some("Example Summary".to_owned())
}

/// Returns an `Option` containing an event location string parsed from `input`.
fn get_location(input: &str) -> Option<String> {
    Some("Example Location".to_owned())
}
