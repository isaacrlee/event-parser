# Event Parser
Natural language date and time parsing library for Rust

![Build](https://github.com/isaacrlee/event-parser/workflows/Build/badge.svg)

Event Parser is a natural language parsing library written in Rust for parsing event names with their corresponding dates and times. It builds on the `chrono` and `regex` crates to delivery a library that provides more extensive coverage of natural language statements. The library is built with 2 crates, `time_parse` for parsing relative times and `date_parse` for parsing dates. Together with the `Recognizable` trait they form the library. 

Can be used to create events in the `ical` format, which is compatible with most calendar applications (Google Calendar, 

## Installation

## Usage example

Creating an `ical` event with the date and time parsers.

Input string: "Lunch at 12pm on 6/15"
```rust
let mut e = Event::new();
let today = Local::today();

if let Some(start_time) = TimeParser::parse(text) {
    if let Some(start_date) = DateParser::parse(text) {
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::new(d, t), Utc);
        dt.with_timezone(&Local);
    }
    e.starts(dt);
    e.ends(dt.checked_add_signed(Duration::hours(1)).unwrap());
}
```

_For more examples and usage, please refer to the [docs](docs)._

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
cargo test
```

## License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)  
### The MIT License
Distributed under the MIT license. See ``LICENSE`` for more information.  
Copyright (c) 2020 Isaac Lee and Alex Grimes
