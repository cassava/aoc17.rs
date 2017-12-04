use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate clap;
use clap::{App, Arg};

extern crate aoc;

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
        2 => day2(),
        3 => day3(),
        _ => println!("Unknown day, try something between 1 and 2."),
    }
}

fn day3() {
    println!("Day 3: the spiral memory challenge.");
    println!(" -> reading number from stdin");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    println!(" -> (a) {}", aoc::SpiralPoint::new(n - 1).manhattan_distance_from_origin());

    // Perform the stress test.
    // TODO: this is super-inefficient!
    let mut i = 1;
    let mut memory = HashMap::new();
    memory.insert(aoc::SpiralPoint::new(0), 1);
    let sum = loop {
        let p = aoc::SpiralPoint::new(i);
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
    let v = read_spreadsheet_from_stdin();
    println!(" -> (a) {}", aoc::spreadsheet_checksum(v.iter()));
    println!(" -> (b) {}", aoc::spreadsheet_checksum_div(v.iter()));
}

fn read_spreadsheet_from_stdin() -> Vec<Vec<u32>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let v = lines.map(|s| s.as_ref().unwrap()
                      .split_whitespace()
                      .map(|x| x.parse::<u32>().unwrap())
                      .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    v
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
