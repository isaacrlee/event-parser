//! # Event Parser: Rust NLP Library
//!
//! Aims to parse unstructered text into [iCalendar Events](../icalendar/struct.Event.html).
//! * Parses text into events with a date and time relative to the local time.
//! * Event Parser defaults to be timezone aware.
//! * Leverages the crate [date_time_parser](../date_time_parser/index.html) for parsing out the dates and time of events.
//!
//! ## Usage
//! Put this in your `Cargo.toml`:
//! ```toml,ignore
//! [dependencies]
//! event_parser = "0.1.0"
//! ```
//! Then put this in your crate root:
//! ```
//! extern crate event_parser;
//! ```
//!
//! ## Example: Dinner at 7pm
//! Pass English natural language that describes an event to the [`to_event`](../event_parser/fn.to_event.html) function to parse the expression. It will parse the expression into the
//! [iCalendar Events](../icalendar/struct.Event.html) format.  
//! 
//! If applicable, the event will have a start and end time, or be classified as an all-day event. Addtionally, a date will be parsed 
//! for the event, defaulting to the current day if no date is found. The event will also have a summary (the name of the event), if one is given.
//! ```
//! use event_parser::to_event;
//! use chrono::{Duration, Local, NaiveTime, NaiveDateTime, NaiveDate};
//! use icalendar::{Component, Event};
//!
//! # fn equal(actual: Event, expected: Event) -> bool {
//! #     return true
//! # }
//!
//! let event = to_event("Dinner at 7");
//! let expected_event = Event::new()
//!     .summary("Dinner")
//!     .starts(Local::today().naive_local().and_hms(19, 0, 0))
//!     .ends(Local::today().naive_local().and_hms(19, 0, 0) + Duration::hours(1))
//!     .done();
//! assert!(equal(event, expected_event));
//! ```
//! 
//! ## Example: Doctor's Appointment
//! The crate parses events relative to the current local time, meaning it's timezone sensitive and allows for events
//! to be parsed without a particular date. Specifying a date like "tomorrow" or "next friday" is enough to 
//! determine the date on which that event is supposed to take place.
//! 
//! Additionally, if no end time is given for an event, the event duration defauls to 1 hour (similar to Google Calendar).
//! 
//! ```
//! use event_parser::to_event;
//! use chrono::{Duration, Local};
//! use icalendar::{Component, Event};
//! 
//! # fn equal(actual: Event, expected: Event) -> bool {
//! #     return true
//! # }
//! 
//! let event = to_event("4pm Doctor's Appointment tomorrow");
//! let expected_event = Event::new()
//!     .summary("Doctor's Appointment")
//!     .starts(Local::today().naive_local().and_hms(16, 0, 0) + Duration::days(1))
//!     .ends(Local::today().naive_local().and_hms(17, 0,0 ) + Duration::days(1))
//!     .done();
//! assert!(equal(event, expected_event));
//! ```
//! 
//! ## Example: Printing
//! Event Parser also provides a [`pretty_print`](../event_parser/fn.pretty_print.html) function to print the 
//! [iCalendar Events](../icalendar/struct.Event.html) that it parses out. This is a convenience function for using 
//! this crate in command line tools to be able to print events to the user.
//! 
//! ```
//! use event_parser::{to_event, pretty_print};
//! use icalendar::{Component, Event};
//! 
//! let event = to_event("Flight on saturday at noon");
//! pretty_print(event);
//! ```
//! Output:
//! ```txt
//! date: 2020-04-25T12:00:00Z
//! Event: "Flight"
//! 12:00pm April 25 2020 - 01:00pm April 25 2020
//! ```
//! 

use chrono::{Date, DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc, Weekday};
use date_time_parser::DateParser;
use date_time_parser::TimeParser;
use icalendar::{Component, Event};
use regex::Regex;



/// An intermediate expression for parsing the start and end of an `Event`. This is a abstract syntax that is used to represent the date, start time, and end time of each event, if given.
enum EventStartAndEndExpr {
    /// An event with unknown date and time
    Unknown,

    /// An event with only a given start time (no date or end time)
    Starts(NaiveTime),

    /// An event with a given start time _and_ end time (no date)
    StartsAndEnds(NaiveTime, NaiveTime),

    /// An event with a start time and date (no end time)
    StartsWithDate(NaiveTime, NaiveDate),

    /// An event with all information, a start time, end time, and date
    StartsAndEndsWithDate(NaiveTime, NaiveTime, NaiveDate),

    /// An event with only a date
    AllDay(NaiveDate),

    /// A multi-day event with a start date and an end date
    AllDayStartsAndEnds(NaiveDate, NaiveDate),
}



/// Parses `text` into an `Event` in `VEVENT` format [(RFC 5545, Section 3.6.1 )](https://tools.ietf.org/html/rfc5545#section-3.6.1).
///
/// # Arguments
///
/// * `text` - A string slice that holds the the text to be parsed.
/// 
/// # Example
/// ```
/// use event_parser::to_event;
/// use chrono::{DateTime, Utc, offset, prelude, NaiveDate};
/// use icalendar::{Component, Event};
/// 
/// # fn equal(actual: Event, expected: Event) -> bool {
/// #     return true
/// # }
///
/// let event = to_event("Summer Camp 6/1-6/8");
/// let expected_event = Event::new()
///     .summary("Dinner")
///     .starts(DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 6, 1).and_hms(0, 0, 0), Utc))
///     .ends(DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 6, 8).and_hms(0, 0, 0), Utc))
///     .done();
/// assert!(equal(event, expected_event));
/// ```
pub fn to_event(text: &str) -> Event {
    let mut e = Event::new();

    let today = Local::today();

    let expr = to_start_end_expr(text);

    match expr {
        EventStartAndEndExpr::Unknown => {
            e.all_day(today);
        }
        EventStartAndEndExpr::Starts(t) => {
            // default to today
            let dt = DateTime::<Utc>::from_utc(NaiveDateTime::new(today.naive_utc(), t), Utc);
            dt.with_timezone(&Local);

            e.starts(dt);
            e.ends(dt.checked_add_signed(Duration::hours(1)).unwrap()); // end is 1 hour after start
        }
        EventStartAndEndExpr::AllDay(d) => {
            e.all_day(Date::<Utc>::from_utc(d, Utc));
        }
        EventStartAndEndExpr::StartsWithDate(t, d) => {
            let dt = DateTime::<Utc>::from_utc(NaiveDateTime::new(d, t), Utc);
            dt.with_timezone(&Local);

            e.starts(dt);
            e.ends(dt.checked_add_signed(Duration::hours(1)).unwrap()); // end is 1 hour after start
        }
        EventStartAndEndExpr::StartsAndEnds(start, end) => {
            // default to today
            let start_dt =
                DateTime::<Utc>::from_utc(NaiveDateTime::new(today.naive_utc(), start), Utc);
            start_dt.with_timezone(&Local);

            let end_dt = DateTime::<Utc>::from_utc(NaiveDateTime::new(today.naive_utc(), end), Utc);
            end_dt.with_timezone(&Local);

            e.starts(start_dt);
            e.ends(end_dt);
        }
        EventStartAndEndExpr::StartsAndEndsWithDate(start, end, d) => {
            let start_dt = DateTime::<Utc>::from_utc(NaiveDateTime::new(d, start), Utc);
            start_dt.with_timezone(&Local);

            let end_dt = DateTime::<Utc>::from_utc(NaiveDateTime::new(d, end), Utc);
            end_dt.with_timezone(&Local);

            e.starts(start_dt);
            e.ends(end_dt);
        }
        EventStartAndEndExpr::AllDayStartsAndEnds(start, end) => {
            e.start_date(Date::<Utc>::from_utc(start, Utc));
            e.end_date(Date::<Utc>::from_utc(end, Utc));
        }
    }

    // location parsing is not yet supported
    // if let Some(loc) = get_location(text) {
    //     e.location(&loc);
    // }

    if let Some(summary) = summary(text) {
        e.summary(&summary);
    }

    e.done()
}

/// Parses `text` with `date_parser` and `time_parser` to return an `Option` containing an `EventStartAndEndExpr`.
fn to_start_end_expr(text: &str) -> EventStartAndEndExpr {
    // Hack: look for {'-', "to"}, if found, then it's a StartsAndEnds, StartsAndEndsWithDate, or AllDayStartsAndEnds
    //  Get expressions before and after {'-', "to"}
    let re = Regex::new(r"(?P<start>[/\w]+)(\s?(-|to)\s?)(?P<end>[/\w]+)").unwrap();
    if let Some(caps) = re.captures(text) {
        if let Some(start_match) = caps.name("start") {
            if let Some(start_time) = TimeParser::parse(start_match.as_str()) {
                if let Some(end_match) = caps.name("end") {
                    if let Some(end_time) = TimeParser::parse(end_match.as_str()) {
                        if let Some(date) = DateParser::parse(text) {
                            return EventStartAndEndExpr::StartsAndEndsWithDate(
                                start_time, end_time, date,
                            );
                        }

                        return EventStartAndEndExpr::StartsAndEnds(start_time, end_time);
                    }
                }
            }

            if let Some(start_date) = DateParser::parse(start_match.as_str()) {
                if let Some(end_match) = caps.name("end") {
                    if let Some(end_date) = DateParser::parse(end_match.as_str()) {
                        return EventStartAndEndExpr::AllDayStartsAndEnds(start_date, end_date);
                    }
                }
            }
        }
    }

    if let Some(start_time) = TimeParser::parse(text) {
        if let Some(start_date) = DateParser::parse(text) {
            return EventStartAndEndExpr::StartsWithDate(start_time, start_date);
        }
        return EventStartAndEndExpr::Starts(start_time);
    }

    if let Some(start_date) = DateParser::parse(text) {
        return EventStartAndEndExpr::AllDay(start_date);
    }

    EventStartAndEndExpr::Unknown
}

/// Returns an `Option` containing an event's summary string parsed from `text`.
fn summary(text: &str) -> Option<String> {
    let mut clean_text = text.to_string();
    // replace all patterns with ""
    let set = vec![
        r"\d{1,2}/(\d{1,2})",                            // dates
        r"(\d{1,2})(/)(\d{1,2})(/)(\d{4}|\d{2})",        // dates
        r"(?i)(^|\b)(\d{1,2}):?(\d{2})?([ap]m?)?($|\b)", // times
        r"(?i)(jan|january|feb|mar|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)\s(?P<date>\d{1,2})?", // month dates
        r"(?i)(mon|tue|wed|thurs|fri|sat|sun)(r?day|r?sday|nesay|urday)?\b", // weekdays
        r"(?i)(next|last|this)\s\w+",                                        // relative words
        r"(?i)\b(at|in|on|from|next|this|last|morning|afternoon|evening|night|noon|afternoon|tomorrow)\b",
        r"(?i)-",
    ]; // words to replace

    for pattern in set {
        let re = Regex::new(pattern).unwrap();
        clean_text = re.replace_all(&clean_text, "").to_string();
    }

    Some(clean_text.trim().to_owned())
}

/// Pretty prints formatted `Event` to the standard output. Returns `Void` and prints to `stdout`.
///
/// # Arguments
///
/// * `event` - An [iCalendar Event](../icalendar/struct.Event.html) to be printed.
///
/// # Example
/// ```
/// use event_parser::{to_event, pretty_print};
/// use icalendar::{Component, Event};
/// 
/// let event = to_event("Lunch at noon next Friday");
/// pretty_print(event);
/// ```
/// Output:
/// ```txt
/// date: 2020-05-01T12:00:00Z
/// Event: "Lunch"
/// 12:00pm May 01 2020 - 01:00pm May 01 2020
/// ```
pub fn pretty_print(e: Event) {
    // if start exists
    //  look for end

    if e.properties().contains_key("SUMMARY") {
        println!(
            "Event: {:?}",
            e.properties().get("SUMMARY").unwrap().value()
        );
    }

    if e.properties().contains_key("LOCATION") {
        println!(
            "Location: {:?}",
            e.properties().get("LOCATION").unwrap().value()
        );
    }

    if e.properties().contains_key("DTSTART") {
        let start_ndt = convert_ical_datetime(&e, "DTSTART");
        if e.properties().contains_key("DTEND") {
            let end_ndt = convert_ical_datetime(&e, "DTEND");
            println!(
                "{} {} - {} {}",
                start_ndt.format("%I:%M%P"),
                start_ndt.format("%B %d %Y"),
                end_ndt.format("%I:%M%P"),
                end_ndt.format("%B %d %Y"),
            );
        }
    }
}

fn convert_ical_datetime(e: &Event, key: &str) -> NaiveDateTime {
    let value = e.properties().get(key).unwrap().value();

    fn to_naive_date(date: iso8601::Date) -> NaiveDate {
        match date {
            iso8601::Date::YMD { year, month, day } => {
                NaiveDate::from_ymd(year, month, day)
            }
            iso8601::Date::Week { year, ww, d } => {
                let mut day = Weekday::Sun;
                for _ in 0..d {
                    day = day.succ();
                }
                NaiveDate::from_isoywd(year, ww, day)
            }
            iso8601::Date::Ordinal { year, ddd } => {
                NaiveDate::from_yo(year, ddd)
            }
        }
    }
    match iso8601::datetime(value) {
        Ok(dt) => {
            NaiveDateTime::new(
                to_naive_date(dt.date),
                NaiveTime::from_hms(dt.time.hour, dt.time.minute, dt.time.second))
        }
        Err(_) => {
            let date = iso8601::date(value).unwrap();
            NaiveDateTime::new(
                to_naive_date(date),
                NaiveTime::from_hms(0, 0, 0))
        }
    }
}

///////////////////////////////
// TESTS
//////////////////////////////

#[cfg(test)]
mod to_event_tests {
    use super::{summary, to_event, convert_ical_datetime};
    use chrono::{prelude::*, Duration, Local, NaiveDate, NaiveDateTime, Weekday};
    #[test]
    fn start_tests() {
        assert_to_event("Lunch at 1pm", time_today(13, 0, 0), time_today(14, 0, 0));
        assert_to_event(
            "Lunch at 12:30pm",
            time_today(12, 30, 0),
            time_today(13, 30, 0),
        );
        assert_to_event("Dinner at 7", time_today(19, 0, 0), time_today(20, 0, 0));
        assert_to_event("Lunch at 12pm", time_today(12, 0, 0), time_today(13, 0, 0));
        assert_to_event("Dinner at 7pm", time_today(19, 0, 0), time_today(20, 0, 0));
        assert_to_event("Flight at noon", time_today(12, 0, 0), time_today(13, 0, 0));
    }

    #[test]
    fn starts_and_ends_tests() {
        assert_to_event("Lunch 1-2", time_today(13, 0, 0), time_today(14, 0, 0));
        assert_to_event("Dinner 7-9pm", time_today(19, 0, 0), time_today(21, 0, 0));
        assert_to_event("Lunch 11-1pm", time_today(11, 0, 0), time_today(13, 0, 0));
    }

    #[test]
    fn starts_and_ends_with_date_tests() {
        let year = Local::now().year();
        assert_to_event(
            "Lunch 1-2pm 6/10",
            time_and_date(13, 0, 0, 6, 10, year),
            time_and_date(14, 0, 0, 6, 10, year),
        )
    }

    #[test]
    fn all_day_tests() {
        let year = Local::now().year();
        assert_to_event_all_day("America's Birthday 7/4", ndt_from_ymd(year, 7, 4));
        assert_to_event_all_day("America's Birthday July 4th", ndt_from_ymd(year, 7, 4));
    }

    #[test]
    fn start_with_date_tests() {
        let year = Local::now().year();
        assert_to_event(
            "Lunch at 1pm 6/15",
            time_and_date(13, 0, 0, 6, 15, year),
            time_and_date(14, 0, 0, 6, 15, year),
        );
    }

    #[test]
    fn all_day_starts_and_ends_tests() {
        let year = Local::now().year();
        assert_to_event(
            "Welcome Week 9/1-9/8",
            ndt_from_ymd(year, 9, 1),
            ndt_from_ymd(year, 9, 8),
        )
    }

    #[test]
    fn get_summary_tests() {
        assert_eq!(
            summary("Lunch at noon next Friday"),
            Some("Lunch".to_owned())
        );
        assert_eq!(
            summary("Dinner with friends tomorrow"),
            Some("Dinner with friends".to_owned())
        );
        assert_eq!(
            summary("My Birthday April 5"),
            Some("My Birthday".to_owned())
        );
        assert_eq!(
            summary("April 5 My Birthday"),
            Some("My Birthday".to_owned())
        );
        assert_eq!(
            summary("6pm Next Friday Doctor's Appointment"),
            Some("Doctor's Appointment".to_owned())
        );
        assert_eq!(
            summary("6pm Doctor's Appointment Next Friday"),
            Some("Doctor's Appointment".to_owned())
        );
        assert_eq!(
            summary("6pm Doctor's Appointment Next Friday"),
            Some("Doctor's Appointment".to_owned())
        );
        assert_eq!(
            summary("Flight on saturday at noon"),
            Some("Flight".to_owned())
        );
        assert_eq!(
            summary("Senior Week 6/17-6/21"),
            Some("Senior Week".to_owned())
        )
    }

    fn ndt_from_ymd(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd(y, m, d).and_hms(0, 0, 0)
    }

    fn time_today(h: u32, m: u32, s: u32) -> NaiveDateTime {
        Local::today().and_hms(h, m, s).naive_local()
    }

    fn time_and_date(h: u32, min: u32, s: u32, mon: u32, d: u32, y: i32) -> NaiveDateTime {
        NaiveDate::from_ymd(y, mon, d).and_hms(h, min, s)
    }

    #[allow(dead_code)]
    fn date_for_friday(h: u32, m: u32, next: bool) -> NaiveDateTime {
        let today_weekday = Local::now().weekday();
        let today_num = today_weekday.number_from_monday() as i64;

        let goal_num = Weekday::Fri.number_from_monday() as i64;

        let diff = goal_num - today_num;
        if diff > 0 {
            let duration;
            if next {
                duration = Duration::days(diff + 7);
            } else {
                duration = Duration::days(diff);
            }
            return Local::today().and_hms(h, m, 0).naive_local() + duration;
        } else if diff == 0 {
            let duration;
            println!("day: {:?}", Local::now().weekday());
            if next {
                duration = Duration::days(14);
            } else {
                duration = Duration::days(7);
            }
            return Local::today().and_hms(h, m, 0).naive_local() + duration;
        } else {
            let pos_diff = 7 + diff;
            let duration;
            if next {
                duration = Duration::days(pos_diff + 7);
            } else {
                duration = Duration::days(pos_diff);
            }
            return Local::today().and_hms(h, m, 0).naive_local() + duration;
        }
    }

    fn assert_to_event_all_day(input: &str, expected_start: NaiveDateTime) {
        let e = to_event(input);

        assert_eq!(
            convert_ical_datetime(&e, "DTSTART"),
            expected_start
        );
    }

    fn assert_to_event(input: &str, expected_start: NaiveDateTime, expected_end: NaiveDateTime) {
        let e = to_event(input);

        let start = convert_ical_datetime(&e, "DTSTART");
        let end = convert_ical_datetime(&e, "DTEND");

        assert_eq!(
            start,
            expected_start
        );

        assert_eq!(
            end,
            expected_end
        );
    }
}
