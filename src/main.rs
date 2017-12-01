use std::io;
use std::io::prelude::*;

extern crate aoc;

#[allow(dead_code)]
fn read_stdin_split_on_space<T: std::str::FromStr>() -> Vec<T> {
    let mut v = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for field in line.unwrap().split(|c: char| c.is_whitespace()) {
            if field.is_empty() {
                continue;
            }
            match field.parse::<T>() {
                Ok(x) => v.push(x),
                _ => continue,
            }
        }
    }
    v
}

fn read_serial_number(s: &str) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v = read_serial_number(input.as_str());
    println!("{}", aoc::inverse_captcha(v));
}
