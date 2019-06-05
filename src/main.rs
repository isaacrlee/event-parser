use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use icalendar::{Component, Event};
use std::io;
use std::io::prelude::*;
// use regex::Regex;
use eventparser::date_parse::DateParser;
use eventparser::time_parse::TimeParser;

// TODO: Generic Read/Write
fn main() {
    println!("e.g. Lunch at 12pm");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let event = parse_input(&line.unwrap());
        println!("{:?}", event);
    }
}

// Examples
// Starts: Lunch (12pm, 12, noon, twelve, at 12}
// AllDay: Dillo Day {Saturday, 6/1, sat, this saturday, next saturday, june 1, june 1st}
// StartsAndEnds: Concert {7-9pm, 7-9, 7 to 9, from 7 to 9, from seven to nine}
// AllDayStartsAndEnds: Welcome Week {9/1-9/8, September {1st-8, 1-8}}

// Structs
/// Abstract expression for the start and end of an event.
enum EventStartAndEndExpr {
    Unknown,
    Starts(NaiveTime),
    StartsAndEnds(NaiveTime, NaiveTime),
    StartsWithDate(NaiveTime, NaiveDate),
    StartsAndEndsWithDate(NaiveTime, NaiveTime, NaiveDate),
    AllDay(NaiveDate),
    AllDayStartsAndEnds(NaiveDate, NaiveDate),
}

/// Parses this string slice into an `Event`.
pub fn parse_input(text: &str) -> Event {
    println!("Input: {}", text);

    let mut e = Event::new();

    let now = Local::now();
    let today = Local::today();

    // start time/date and end time/date
    let expr = get_start_and_end(text);

    // use EventStartAndEndExpr::*;

    match expr {
        EventStartAndEndExpr::Unknown => {
            e.all_day(today);
        }
        EventStartAndEndExpr::Starts(t) => {
            // TODO: check if time is later than now => set day to tomorrow, else, set day to today
            // default to today
            let ndt = NaiveDateTime::new(today.naive_utc(), t);
            let dt = DateTime::<Utc>::from_utc(ndt, Utc); // TODO: Local
            println!("dt: {}", dt);
            e.starts(dt);
        }
        _ => {}
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
fn get_start_and_end(text: &str) -> EventStartAndEndExpr {
    // Hack: look for {'-', "to"}, if found, then it's a StartsAndEnds, StartsAndEndsWithDate, or AllDayStartsAndEnds
    //  Get expressions before and after {'-', "to"}

    if let Some(start_time) = TimeParser::parse(text).unwrap() {
        // if let Some(start_date) = DateParser::parse(text).unwrap() {
        //     return EventStartAndEndExpr::StartsWithDate(start_time, start_date);
        // }
        return EventStartAndEndExpr::Starts(start_time);
    }

    // if let Some(start_date) = DateParser::parse(text).unwrap() {
    //     return EventStartAndEndExpr::AllDay(start_date);
    // }

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
    EventStartAndEndExpr::Unknown
}

/// Returns an `Option` containing an event's summary string parsed from `text`.
fn get_summary(text: &str) -> Option<String> {
    Some("Example Summary".to_owned())
}

/// Returns an `Option` containing an event location string parsed from `text `.
fn get_location(text: &str) -> Option<String> {
    Some("Example Location".to_owned())
}
