extern crate chrono;
extern crate icalendar;
extern crate regex;

use icalendar::{Component, Event};
use chrono::{NaiveDate, NaiveTime};
use regex::Regex;

/// Parses input string into Event
pub fn parse_input(input: &str) -> Event {

    println!("Event: {}", input);
    let mut e = Event::new();

    e.summary("My Birthday");

    e.done()

    // Regex Stuff

    // summary

    // starts
    // ends (optional)

    // start_date
    // end_date (optional)

    // all_day

    // location

    // Create DateTime

}

fn get_date(input: &str) -> Option<NaiveDate> {
    unimplemented!();
}

pub fn get_time(input: &str) -> Option<NaiveTime> {
    let mut re = Regex::new(r"(\d+:\d+)").unwrap();
    let mut mat = re.find(input);

    if let Some(s) = mat {
        let mut iter = s.as_str().split(":");
        let h = iter.next().unwrap().parse().unwrap();
        let m = iter.next().unwrap().parse().unwrap();
        return Some(NaiveTime::from_hms(h, m, 0));
    }

    // re = Regex::new(r"(\d+:\d+)").unwrap();
    // mat = re.find(input).unwrap()

    // unimplemented!()
    None
}

fn get_summary(input: &str) -> Option<String> {
    unimplemented!();
}

fn get_location(input: &str) -> Option<String> {
    unimplemented!();
}

mod test {
    use super::get_time;
    use chrono::NaiveTime;

    #[test]
    fn simple_time_test() {
        // let input = String::from("Lunch on Friday at 12pm");
        // let time = get_time(&input).unwrap();
        // assert_eq!(time, NaiveTime::from_hms(12, 0, 0));

        let input = String::from("Breakfast on Friday at 10:30");
        let time = get_time(&input).unwrap();
        assert_eq!(time, NaiveTime::from_hms(10, 30, 0));
    }
}

