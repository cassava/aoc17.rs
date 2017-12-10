extern crate aoc;

use std::collections::HashMap;
use std::fmt;

fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 6: {}", PUZZLE);

    let nums: Vec<u32> = input.to_str()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut mem = Memory::from_iter(nums.iter());

    let mut count = 0;
    println!("Balancing:");
    while !mem.is_known() {
        mem.balance();
        println!(" -> {}", mem);
        count += 1;
    }

    println!(":: Answer 1 is {}", count);
    println!(":: Answer 2 is {}", mem.known_from().unwrap());
}

#[derive(Debug)]
struct Memory{
    banks: Vec<u32>,
    balanced: u32,
    history: HashMap<String, u32>,
}

impl Memory {
    fn from_iter<'a, I: Iterator<Item = &'a u32>>(it: I)-> Self {
        Memory{
            banks: it.map(|x| *x).collect(),
            balanced: 0,
            history: HashMap::new(),
        }
    }

    fn balance(&mut self) {
        self.register();
        let (mut idx, mut max) = self.banks.iter().enumerate().fold((0, 0), |acc, x| {
            if *x.1 > acc.1 {
                (x.0, *x.1)
            } else {
                acc
            }
        });

        let n = self.banks.len();
        self.banks[idx] = 0;
        while max != 0 {
            idx = (idx + 1) % n;
            self.banks[idx] += 1;
            max -= 1;
        }
        self.balanced += 1;
    }

    fn is_known(&self) -> bool {
        self.history.contains_key(&self.to_string())
    }

    fn known_from(&self) -> Option<u32> {
        match self.history.get(&self.to_string()) {
            Some(n) => Some(self.balanced - *n),
            None => None,
        }
    }

    fn register(&mut self) {
        let state = self.to_string();
        self.history.insert(state, self.balanced);
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("[");
        for x in self.banks.iter() {
            output.push_str(x.to_string().as_str());
            output.push_str(" ");
        }
        if output.len() > 1 {
            output.pop();
        }
        output.push_str("]");
        write!(f, "{} ", output)
    }
}

const PUZZLE: &'static str = "Memory Reallocation";
const INPUT: &'static str = r"
0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11
";
