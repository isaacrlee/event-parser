use eventparser::parse_input;
// use std::io;
// use std::io::prelude::*;

fn main() {
    let e = parse_input("Test");
    println!("Event: {:?}", e);
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     parse_input(line.unwrap());
    // }
}
