use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

extern crate clap;
use clap::{App, Arg};

extern crate aoc;

fn main() {
    let matches = App::new("Advent of Code 2017")
        .version("0.4.0")
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
        2 => day2(),
        3 => day3(),
        4 => day4(),
        _ => println!("Unknown day, try something between 1 and 2."),
    }
}

fn day4() {
    println!("Day 4: the passphrase challenge.");
    println!(" -> reading lines from stdin");
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let count = lines.iter().fold(0, |acc, x| if aoc::passphrase::is_valid(x.as_str()) { acc + 1 } else { acc });
    println!(" -> (a) {}", count);
    let count = lines.iter().fold(0, |acc, x| if aoc::passphrase::is_supervalid(x.as_str()) { acc + 1 } else { acc });
    println!(" -> (b) {}", count);
}

fn day3() {
    println!("Day 3: the spiral memory challenge.");
    println!(" -> reading number from stdin");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    println!(" -> (a) {}", aoc::spiral::Point::new(n - 1).manhattan_distance_from_origin());

    // Perform the stress test.
    // TODO: this is super-inefficient!
    let mut i = 1;
    let mut memory = HashMap::new();
    memory.insert(aoc::spiral::Point::new(0), 1);
    let sum = loop {
        let p = aoc::spiral::Point::new(i);
        let sum = p.neighbours().iter().fold(0, |acc, x| if let Some(v) = memory.get(x) { acc + v } else { acc });
        memory.insert(p, sum);
        if sum > n {
            break sum;
        }
        i += 1;
    };
    println!(" -> (b) {}", sum);
}

fn day2() {
    println!("Day 2: the spreadsheet checksum.");
    println!(" -> reading from stdin");
    let v = aoc::spreadsheet::read_from_stdin();
    println!(" -> (a) {}", aoc::spreadsheet::checksum(v.iter()));
    println!(" -> (b) {}", aoc::spreadsheet::checksum_div(v.iter()));
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

fn read_serial_number(s: &str) -> Vec<u32> {
    s.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap()).collect()
}
