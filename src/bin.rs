//! # Event Parser
//!
//! A command line tool for parsing unstructered text into [iCalendar Events](../icalendar/struct.Event.html) using the [event_parser](../event_parser/index.html) library.

use event_parser;
use std::io::BufRead;

fn main() {
    println!("e.g. Lunch at 12pm");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let event = event_parser::to_event(&line.unwrap());
        event_parser::pretty_print(event);
    }
}
