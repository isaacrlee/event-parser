//! # Event Parser
//!
//! A utility for parsing unstructered text into [iCalendar Events](../icalendar/struct.Event.html) using the [datetimeparser](../datetimeparser/index.html) library.


use chrono::{Date, DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use datetimeparser::date_parse::DateParser;
use datetimeparser::time_parse::TimeParser;
use icalendar::{Component, Event};
use regex::Regex;
use std::io::BufRead;

fn main() {
    println!("e.g. Lunch at 12pm");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let event = to_event(&line.unwrap());
        // event.print();
        pretty_print(event);
    }
}

/// An intermediate expression for parsing the start and end of an `Event`.
enum EventStartAndEndExpr {
    Unknown,
    Starts(NaiveTime),
    StartsAndEnds(NaiveTime, NaiveTime),
    StartsWithDate(NaiveTime, NaiveDate),
    StartsAndEndsWithDate(NaiveTime, NaiveTime, NaiveDate),
    AllDay(NaiveDate),
    AllDayStartsAndEnds(NaiveDate, NaiveDate),
}

/// Parses `text` into an `Event`.
/// # Example
/// ```
/// use super::to_event;
/// let event = to_event("Lunch at 12pm");
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

/// Returns an `Option` containing an `EventStartAndEndExpr`.
/// # Example
/// ```
/// use super::{EventStartAndEndExpr, to_start_end_expr};
/// use chrono::NaiveTime;
///
/// let expr = to_start_and_end_expr("3-5pm");
/// let start = NaiveTime::from_hms(15, 0, 0);
/// let end = NaiveTime::from_hms(15, 0, 0);
/// assert_eq!(expr, EventStartAndEndExpr::StartAndEnd(start, end));
/// ```
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
    let set = vec![r"\d{1,2}/(\d{1,2})", // dates
    r"(\d{1,2})(/)(\d{1,2})(/)(\d{4}|\d{2})", // dates
    r"(?i)(^|\b)(\d{1,2}):?(\d{2})?([ap]m?)?($|\b)", // times
    r"(?i)(jan|january|feb|mar|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)\s(?P<date>\d{1,2})?", // month dates
    r"(?i)(mon|tue|wed|thurs|fri|sat|sun)(r?day|r?sday|nesay|urday)?\b", // weekdays
    r"(?i)(next|last|this)\s\w+", // relative words
    r"(?i)\b(at|in|on|from|next|this|last|morning|afternoon|evening|night|noon|afternoon|tomorrow)\b",
    r"(?i)-"
    ]; // words to replace

    for pattern in set {
        let re = Regex::new(pattern).unwrap();
        clean_text = re.replace_all(&clean_text, "").to_string();
    }

    Some(clean_text.trim().to_owned())
}

/// Pretty prints formatted `Event` to the standard output.
fn pretty_print(e: Event) {
    // if start exists
    //  look for end

    if let Some(summary) = e.properties().get("SUMMARY") {
        let mut summary_string = String::new();
        summary.fmt_write(&mut summary_string).unwrap();
        println!(
            "Event: {:?}",
            event_property_line_to_value_str(&summary_string, "SUMMARY")
        );
    }

    if let Some(loc) = e.properties().get("LOCATION") {
        let mut loc_string = String::new();
        loc.fmt_write(&mut loc_string).unwrap();
        println!(
            "Location: {:?}",
            event_property_line_to_value_str(&loc_string, "LOCATION")
        );
    }

    if let Some(start) = e.properties().get("DTSTART") {
        let mut start_string = String::new();

        start.fmt_write(&mut start_string).unwrap();

        let start_ndt = event_property_line_to_ndt(&start_string, "DTSTART").unwrap();
        if let Some(end) = e.properties().get("DTEND") {
            let mut end_string = String::new();
            end.fmt_write(&mut end_string).unwrap();
            if let Some(end_ndt) = event_property_line_to_ndt(&end_string, "DTEND") {
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
}

/// Helper function for getting the value of an event's `property` and converting it to a `NaiveDateTime`.
fn event_property_line_to_ndt(s: &str, property: &str) -> Option<NaiveDateTime> {
    // TODO: Handle all day
    match NaiveDateTime::parse_from_str(
        event_property_line_to_value_str(s, property),
        "%Y%m%dT%H%M%S",
    ) {
        Ok(res) => Some(res),
        Err(_) => {
            match NaiveDate::parse_from_str(
                event_property_line_to_date_value_str(s, property),
                "%Y%m%d",
            ) {
                Ok(res) => Some(res.and_hms(0, 0, 0)),
                Err(_) => None,
            }
        }
    }
}

/// Helper function for getting the value of an event's `property`.
fn event_property_line_to_value_str<'a>(s: &'a str, property: &str) -> &'a str {
    s.trim().get(property.len() + 1..).unwrap()
}

/// Helper function for getting the value of an event's `property` when value is a date.
fn event_property_line_to_date_value_str<'a>(s: &'a str, property: &str) -> &'a str {
    s.trim()
        .get(property.len() + ";VALUE=DATE:".len()..)
        .unwrap()
}

///////////////////////////////
// TESTS
//////////////////////////////

#[cfg(test)]
mod to_event_tests {
    use super::{event_property_line_to_ndt, summary, to_event};
    use chrono::{Local, NaiveDate, NaiveDateTime};
    use icalendar::Component;
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
        assert_to_event(
            "Flight on saturday at noon",
            time_and_date(12, 0, 0, 6, 15, 2019),
            time_and_date(13, 0, 0, 6, 15, 2019),
        );
    }

    #[test]
    fn starts_and_ends_tests() {
        assert_to_event("Lunch 1-2", time_today(13, 0, 0), time_today(14, 0, 0));
        assert_to_event("Dinner 7-9pm", time_today(19, 0, 0), time_today(21, 0, 0));
        assert_to_event("Lunch 11-1pm", time_today(11, 0, 0), time_today(13, 0, 0));
    }

    #[test]
    fn starts_and_ends_with_date_tests() {
        assert_to_event(
            "Lunch 1-2pm 6/10",
            time_and_date(13, 0, 0, 6, 10, 2019),
            time_and_date(14, 0, 0, 6, 10, 2019),
        )
    }

    #[test]
    fn all_day_tests() {
        assert_to_event_all_day("America's Birthday 7/4", ndt_from_ymd(2019, 7, 4));
        assert_to_event_all_day("America's Birthday July 4th", ndt_from_ymd(2019, 7, 4));
    }

    #[test]
    fn start_with_date_tests() {
        assert_to_event(
            "Lunch at 1pm 6/15",
            time_and_date(13, 0, 0, 6, 15, 2019),
            time_and_date(14, 0, 0, 6, 15, 2019),
        );
        assert_to_event(
            "Lunch at 1pm next Friday",
            time_and_date(13, 0, 0, 6, 21, 2019),
            time_and_date(14, 0, 0, 6, 21, 2019),
        );
    }

    #[test]
    fn all_day_starts_and_ends_tests() {
        assert_to_event(
            "Welcome Week 9/1-9/8",
            ndt_from_ymd(2019, 9, 1),
            ndt_from_ymd(2019, 9, 8),
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

    fn assert_to_event_all_day(input: &str, expected_start: NaiveDateTime) {
        let e = to_event(input);

        let start = e.properties().get("DTSTART").unwrap();
        let mut start_string = String::new();
        start.fmt_write(&mut start_string).unwrap();

        assert_eq!(
            event_property_line_to_ndt(&start_string, "DTSTART").unwrap(),
            expected_start
        );
    }

    fn assert_to_event(input: &str, expected_start: NaiveDateTime, expected_end: NaiveDateTime) {
        let e = to_event(input);

        let start = e.properties().get("DTSTART").unwrap();
        let end = e.properties().get("DTEND").unwrap();

        let mut start_string = String::new();
        start.fmt_write(&mut start_string).unwrap();

        let mut end_string = String::new();
        end.fmt_write(&mut end_string).unwrap();

        assert_eq!(
            event_property_line_to_ndt(&start_string, "DTSTART").unwrap(),
            expected_start
        );

        assert_eq!(
            event_property_line_to_ndt(&end_string, "DTEND").unwrap(),
            expected_end
        );
    }
}
