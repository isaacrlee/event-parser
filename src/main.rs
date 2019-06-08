use chrono::{Date, DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use eventparser::date_parse::DateParser;
use eventparser::time_parse::TimeParser;
use icalendar::{Component, Event, Property};
use regex::Regex;
use std::fmt;
use std::io::{self, prelude::*, BufRead, BufReader, Error, Read, Write};

// TODO: Generic Read/Write

fn main() {
    println!("e.g. Lunch at 12pm");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let event = parse_input(&line.unwrap());
        // println!("{:?}", event);
        // pretty_print(event);
        event.print();
        //println!("{:?}", event.properties().values());

        pretty_print(event);
    }
}

// Examples
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

// Parse Function

/// Parses input string into Event
/// ```
/// use super::parse_input(text: &str);
/// let event = parse_input("Lunch at 12pm");
/// ```
pub fn parse_input(text: &str) -> Event {
    // println!("Input: {}", text);

    let mut e = Event::new();

    let now_dt: DateTime<Local> = Local::now();
    let today = Local::today();

    //let offset = now_dt.offset_from_utc_datetime(now_dt.naive_utc());

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

            // println!("dt: {}", dt);
            e.starts(dt);
            let d = Duration::hours(1);
            e.ends(dt.checked_add_signed(d).unwrap());
        }
        EventStartAndEndExpr::AllDay(d) => {
            let date = Date::<Utc>::from_utc(d, Utc);
            e.all_day(date);
        }
        EventStartAndEndExpr::StartsWithDate(t, d) => {
            let ndt = NaiveDateTime::new(d, t);
            let dt = DateTime::<Utc>::from_utc(ndt, Utc);
            dt.with_timezone(&Local);

            e.starts(dt);
            let d = Duration::hours(1);
            e.ends(dt.checked_add_signed(d).unwrap());
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
        // println!("start time: {}", start_time);
        if let Some(start_date) = DateParser::parse(text).unwrap() {
            return EventStartAndEndExpr::StartsWithDate(start_time, start_date);
        }
        return EventStartAndEndExpr::Starts(start_time);
    }

    if let Some(start_date) = DateParser::parse(text).unwrap() {
        // println!("all day case");
        return EventStartAndEndExpr::AllDay(start_date);
    }

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

/// Returns an `Option` containing an event's summary string parsed from `input`.
fn get_summary(text: &str) -> Option<String> {
    Some("Example Summary".to_owned())
}

/// Returns an `Option` containing an event location string parsed from `input`.
fn get_location(text: &str) -> Option<String> {
    Some("Example Location".to_owned())
}

/// Pretty prints formatted `Event` to the standard output.
fn pretty_print(e: Event) {
    // if start exists
    //  look for end

    if let Some(summary) = e.properties().get("SUMMARY") {
        let mut summary_string = String::new();
        summary.fmt_write(&mut summary_string).unwrap();
        println!("Event: {:?}", parse_property(&summary_string, "SUMMARY"));
    }

    if let Some(loc) = e.properties().get("LOCATION") {
        let mut loc_string = String::new();
        loc.fmt_write(&mut loc_string).unwrap();
        println!("Location: {:?}", parse_property(&loc_string, "LOCATION"));
    }

    if let Some(start) = e.properties().get("DTSTART") {
        let mut start_string = String::new();

        start.fmt_write(&mut start_string).unwrap();

        let start_ndt = parse_property_to_ndt(&start_string, "DTSTART").unwrap();
        //println!("GOT HERE");
        if let Some(end) = e.properties().get("DTEND") {
            let mut end_string = String::new();
            end.fmt_write(&mut end_string).unwrap();
            if let Some(end_ndt) = parse_property_to_ndt(&end_string, "DTEND") {
                println!(
                    "{}-{} {}",
                    start_ndt.format("%I:%M%P"),
                    end_ndt.format("%I:%M%P"),
                    start_ndt.format("%B %d %Y"),
                );
            }
        }
    }
}

pub fn parse_property_to_ndt(s: &str, property: &str) -> Option<NaiveDateTime> {
    // TODO: Handle all day
    match NaiveDateTime::parse_from_str(parse_property(s, property), "%Y%m%dT%H%M%S") {
        Ok(res) => Some(res),
        Err(e) => {
            match NaiveDate::parse_from_str(parse_property_date_only(s, property), "%Y%m%d") {
                Ok(res) => Some(res.and_hms(0, 0, 0)),
                Err(r) => None, // TODO: Error
            }
        }
    }
}

pub fn parse_property<'a>(s: &'a str, property: &str) -> &'a str {
    s.trim().get(property.len() + 1..).unwrap()
}

pub fn parse_property_date_only<'a>(s: &'a str, property: &str) -> &'a str {
    s.trim()
        .get(property.len() + ";VALUE=DATE:".len()..)
        .unwrap()
}

#[cfg(test)]
mod parse_input_tests {
    use super::{parse_input, parse_property_to_ndt, pretty_print};
    use chrono::{Local, NaiveDate, NaiveDateTime, Utc};
    use icalendar::{Component, Event};
    #[test]
    fn start_tests() {
        assert_parse_input("Lunch at 1pm", time_today(13, 0, 0), time_today(14, 0, 0));
        assert_parse_input(
            "Lunch at 12:30pm",
            time_today(12, 30, 0),
            time_today(13, 30, 0),
        );
        assert_parse_input("Dinner at 7", time_today(19, 0, 0), time_today(20, 0, 0));
    }

    #[test]
    fn all_day_tests() {
        assert_parse_input_all_day("America's Birthday 7/4", ndt_from_ymd(2019, 7, 4))
    }

    // #[test]
    // fn start_with_date_tests() {
    //     assert_parse_input(
    //         "Lunch at 1pm 6/15",
    //         time_today(13, 0, 0),
    //         time_today(14, 0, 0),
    //     );
    //     assert_parse_input("Lunch at 12pm ", time_today(12, 0, 0), time_today(13, 0, 0));
    //     assert_parse_input("Dinner at 7pm", time_today(7, 0, 0), time_today(8, 0, 0))
    // }

    fn ndt_from_ymd(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd(y, m, d).and_hms(0, 0, 0)
    }

    fn time_today(h: u32, m: u32, s: u32) -> NaiveDateTime {
        Local::today().and_hms(h, m, s).naive_local()
    }

    fn assert_parse_input_all_day(input: &str, expected_start: NaiveDateTime) {
        let e = parse_input(input);

        let start = e.properties().get("DTSTART").unwrap();
        let mut start_string = String::new();
        start.fmt_write(&mut start_string).unwrap();

        assert_eq!(
            parse_property_to_ndt(&start_string, "DTSTART").unwrap(),
            expected_start
        );
    }

    fn assert_parse_input(input: &str, expected_start: NaiveDateTime, expected_end: NaiveDateTime) {
        let e = parse_input(input);

        let start = e.properties().get("DTSTART").unwrap();
        let end = e.properties().get("DTEND").unwrap();

        let mut start_string = String::new();
        start.fmt_write(&mut start_string).unwrap();

        let mut end_string = String::new();
        end.fmt_write(&mut end_string).unwrap();

        pretty_print(e);

        assert_eq!(
            parse_property_to_ndt(&start_string, "DTSTART").unwrap(),
            expected_start
        );

        assert_eq!(
            parse_property_to_ndt(&end_string, "DTEND").unwrap(),
            expected_end
        );
    }
}
