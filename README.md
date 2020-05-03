# Event Parser
Natural language date, time and event parsing libraries for Rust

![Build](https://github.com/isaacrlee/event-parser/workflows/Build/badge.svg) ![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-blue)

Event Parser contains an date and time natural language parsing library, and event parsing library, as well as a client that demonstrates how to leverage these crates for a simple command-line tool. Written in Rust, the libraries build on the [`chrono`](https://docs.rs/chrono/0.4.11/chrono/) and [`regex`](https://docs.rs/regex/1.3.7/regex/) crates to deliver a library that provides more extensive coverage of natural language statements.

# Date Time Parser: Rust NLP Library

It aims to parse unstructered text into [`NaiveDate`](https://docs.rs/chrono/0.4.11/chrono/naive/struct.NaiveDate.html) and [`NaiveTime`](https://docs.rs/chrono/0.4.11/chrono/naive/struct.NaiveTime.html) formats.

* Date Time Parser has the ability to be timezone aware, but defaults to UTC.
* Allows for parse to be relative to current date/time, or relative to a custom date/time.

## Usage

Put this in your `Cargo.toml`:
```toml
[dependencies]
date_time_parser = "0.1.0"
```

Then put this in your crate root:
```rust
extern crate date_time_parser;
```

## Example: Find a Date

General use of this package involves passing English natural language that includes a date to the `DateParser` struct to parse the expression. If a date is found, it will parse the expression into the `NaiveDate` format.

```rust
use date_time_parser::DateParser;
use chrono::NaiveDate;

let date = DateParser::parse("Lunch on June 5th");
assert_eq!(date, Some(NaiveDate::from_ymd(2020, 6, 5)));
```

_For more examples and usage, please refer to the [docs](https://docs.rs/date_time_parser/0.1.0/date_time_parser/)._

# Event Parser: Rust NLP Library

Aims to parse unstructered text into `iCalendar Events`.

* Parses text into events with a date and time relative to the local time.
* Event Parser defaults to be timezone aware.
* Leverages the crate [`date_time_parser`](https://docs.rs/date_time_parser/0.1.0/date_time_parser/) for parsing out the dates and time of events.

## Usage

Put this in your `Cargo.toml`:
```toml
[dependencies]
event_parser = "0.1.0"
```

Then put this in your crate root:
```rust
extern crate event_parser;
```

## Example: Dinner at 7pm

Pass English natural language that describes an event to the `to_event` function to parse the expression. It will parse the expression into the `iCalendar Events` format.

If applicable, the event will have a start and end time, or be classified as an all-day event. Addtionally, a date will be parsed for the event, defaulting to the current day if no date is found. The event will also have a summary (the name of the event), if one is given.

```rust
use event_parser::to_event;
use chrono::{Duration, Local};
use icalendar::{Component, Event};
 

let event = to_event("Dinner at 7");
let expected_event = Event::new()
    .summary("Dinner")
    .starts(Local::today().and_hms(19, 0, 0))
    .ends(Local::today().and_hms(19, 0, 0) + Duration::hours(1))
    .done();
assert!(equal(event, expected_event));
```

_For more examples and usage, please refer to the [docs](https://docs.rs/event_parser/0.1.0/event_parser/)._

# Command Line Tool

To play around with what is possible with the `event_parser` library, we've provided a command-line tool to be able to test different inputs easily. Simply download the repo and from the root of the project run `cargo run` and type an natural English language event to see how it is parsed!

## Development setup

Install the Rust programming language, and then clone this repository.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/isaacrlee/event-parser.git
```

To run the program and easily play with inputs:

```
cargo run
```

To run the test suite:
```
cargo test --all
```

## License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)  
### The MIT License
Distributed under the MIT license. See ``LICENSE`` for more information.  
Copyright (c) 2020 Isaac Lee and Alex Grimes
