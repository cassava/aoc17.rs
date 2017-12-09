use std::collections::HashMap;
use std::fmt;

fn day6() {
    println!("Day 6: the memory balance challenge.");
    println!(" -> reading line from stdin");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut mem = aoc::memory::Memory::from_iter(nums.iter());

    let mut count = 0;
    println!("Balancing:");
    while !mem.is_known() {
        mem.balance();
        println!(" -> {}", mem);
        count += 1;
    }

    println!("Answer:");
    println!(" -> (a) {}", count);
    println!(" -> (b) {}", mem.known_from().unwrap());
}

#[derive(Debug)]
pub struct Memory{
    banks: Vec<u32>,
    balanced: u32,
    history: HashMap<String, u32>,
}

impl Memory {
    pub fn from_iter<'a, I: Iterator<Item = &'a u32>>(it: I)-> Self {
        Memory{
            banks: it.map(|x| *x).collect(),
            balanced: 0,
            history: HashMap::new(),
        }
    }

    pub fn balance(&mut self) {
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

    pub fn is_known(&self) -> bool {
        self.history.contains_key(&self.to_string())
    }

    pub fn known_from(&self) -> Option<u32> {
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
