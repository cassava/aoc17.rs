use std::io;

extern crate clap;
use clap::{App, Arg};

extern crate aoc;

fn read_serial_number(s: &str) -> Vec<u32> {
    s.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap()).collect()
}

fn day1() {
    println!("Day 1: the inverse captcha.");

    println!(" -> reading from stdin");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v = read_serial_number(input.as_str());

    println!(" -> (a) {}", aoc::inverse_captcha(v.iter(), 1));
    println!(" -> (b) {}", aoc::inverse_captcha(v.iter(), v.len() / 2));
}

fn main() {
    let matches = App::new("Advent of Code 2017")
        .version("0.2.0")
        .author("Ben Morgan <neembi@gmail.com")
        .arg(Arg::with_name("DAY")
             .required(true)
             .help("Which day to run")
             .index(1))
        .get_matches();

    let day = matches.value_of("DAY").unwrap();
    let day = day.parse::<u32>().unwrap_or(0);
    match day {
        1 => day1(),
        _ => println!("Unknown day, try something between 1 and 2."),
    }
}
