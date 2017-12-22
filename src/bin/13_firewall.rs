/*!
# Day 13: Packet Scanners

You need to cross a vast firewall. The firewall consists of several layers, each
with a security scanner that moves back and forth across the layer. To succeed,
you must not be detected by a scanner.

By studying the firewall briefly, you are able to record (in your puzzle input)
the depth of each layer and the range of the scanning area for the scanner
within it, written as depth: range. Each layer has a thickness of exactly 1.
A layer at depth 0 begins immediately inside the firewall; a layer at depth
1 would start immediately after that.

For example, suppose you've recorded the following:

```text
0: 3
1: 2
4: 4
6: 4
```

This means that there is a layer immediately inside the firewall (with range 3),
a second layer immediately after that (with range 2), a third layer which begins
at depth 4 (with range 4), and a fourth layer which begins at depth 6 (also with
range 4). Visually, it might look like this:

```text
 0   1   2   3   4   5   6
[ ] [ ] ... ... [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[ ]             [ ]     [ ]
                [ ]     [ ]
```

Within each layer, a security scanner moves back and forth within its range.
Each security scanner starts at the top and moves down until it reaches the
bottom, then moves up until it reaches the top, and repeats. A security scanner
takes one picosecond to move one step. Drawing scanners as S, the first few
picoseconds look like this:

```text
Picosecond 0:
 0   1   2   3   4   5   6
[S] [S] ... ... [S] ... [S]
[ ] [ ]         [ ]     [ ]
[ ]             [ ]     [ ]
                [ ]     [ ]

Picosecond 1:
 0   1   2   3   4   5   6
[ ] [ ] ... ... [ ] ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]

Picosecond 2:
 0   1   2   3   4   5   6
[ ] [S] ... ... [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[S]             [S]     [S]
                [ ]     [ ]

Picosecond 3:
 0   1   2   3   4   5   6
[ ] [ ] ... ... [ ] ... [ ]
[S] [S]         [ ]     [ ]
[ ]             [ ]     [ ]
                [S]     [S]
```

Your plan is to hitch a ride on a packet about to move through the firewall. The
packet will travel along the top of each layer, and it moves at one layer per
picosecond. Each picosecond, the packet moves one layer forward (its first move
takes it into layer 0), and then the scanners move one step. If there is
a scanner at the top of the layer as your packet enters it, you are caught. (If
a scanner moves into the top of its layer while you are there, you are not
caught: it doesn't have time to notice you before you leave.) If you were to do
this in the configuration above, marking your current position with parentheses,
your passage through the firewall would look like this:

```text
Initial state:
 0   1   2   3   4   5   6
[S] [S] ... ... [S] ... [S]
[ ] [ ]         [ ]     [ ]
[ ]             [ ]     [ ]
                [ ]     [ ]

Picosecond 0:
 0   1   2   3   4   5   6
(S) [S] ... ... [S] ... [S]
[ ] [ ]         [ ]     [ ]
[ ]             [ ]     [ ]
                [ ]     [ ]

 0   1   2   3   4   5   6
( ) [ ] ... ... [ ] ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]


Picosecond 1:
 0   1   2   3   4   5   6
[ ] ( ) ... ... [ ] ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] (S) ... ... [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[S]             [S]     [S]
                [ ]     [ ]


Picosecond 2:
 0   1   2   3   4   5   6
[ ] [S] (.) ... [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[S]             [S]     [S]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [ ] (.) ... [ ] ... [ ]
[S] [S]         [ ]     [ ]
[ ]             [ ]     [ ]
                [S]     [S]


Picosecond 3:
 0   1   2   3   4   5   6
[ ] [ ] ... (.) [ ] ... [ ]
[S] [S]         [ ]     [ ]
[ ]             [ ]     [ ]
                [S]     [S]

 0   1   2   3   4   5   6
[S] [S] ... (.) [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[ ]             [S]     [S]
                [ ]     [ ]


Picosecond 4:
 0   1   2   3   4   5   6
[S] [S] ... ... ( ) ... [ ]
[ ] [ ]         [ ]     [ ]
[ ]             [S]     [S]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [ ] ... ... ( ) ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]


Picosecond 5:
 0   1   2   3   4   5   6
[ ] [ ] ... ... [ ] (.) [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [S] ... ... [S] (.) [S]
[ ] [ ]         [ ]     [ ]
[S]             [ ]     [ ]
                [ ]     [ ]


Picosecond 6:
 0   1   2   3   4   5   6
[ ] [S] ... ... [S] ... (S)
[ ] [ ]         [ ]     [ ]
[S]             [ ]     [ ]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [ ] ... ... [ ] ... ( )
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]
```

In this situation, you are caught in layers 0 and 6, because your packet entered
the layer when its scanner was at the top when you entered it. You are not
caught in layer 1, since the scanner moved into the top of the layer once you
were already there.

The severity of getting caught on a layer is equal to its depth multiplied by
its range. (Ignore layers in which you do not get caught.) The severity of the
whole trip is the sum of these values. In the example above, the trip severity
is 0*3 + 6*4 = 24.

Given the details of the firewall you've recorded, if you leave immediately,
what is the severity of your whole trip?

## Part Two

Now, you need to pass through the firewall without being caught - easier said
than done.

You can't control the speed of the packet, but you can delay it any number of
picoseconds. For each picosecond you delay the packet before beginning your
trip, all security scanners move one step. You're not in the firewall during
this time; you don't enter layer 0 until you stop delaying the packet.

In the example above, if you delay 10 picoseconds (picoseconds 0 - 9), you won't
get caught:

```text
State after delaying:
 0   1   2   3   4   5   6
[ ] [S] ... ... [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[S]             [S]     [S]
                [ ]     [ ]

Picosecond 10:
 0   1   2   3   4   5   6
( ) [S] ... ... [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[S]             [S]     [S]
                [ ]     [ ]

 0   1   2   3   4   5   6
( ) [ ] ... ... [ ] ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]


Picosecond 11:
 0   1   2   3   4   5   6
[ ] ( ) ... ... [ ] ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]

 0   1   2   3   4   5   6
[S] (S) ... ... [S] ... [S]
[ ] [ ]         [ ]     [ ]
[ ]             [ ]     [ ]
                [ ]     [ ]


Picosecond 12:
 0   1   2   3   4   5   6
[S] [S] (.) ... [S] ... [S]
[ ] [ ]         [ ]     [ ]
[ ]             [ ]     [ ]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [ ] (.) ... [ ] ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]


Picosecond 13:
 0   1   2   3   4   5   6
[ ] [ ] ... (.) [ ] ... [ ]
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [S] ... (.) [ ] ... [ ]
[ ] [ ]         [ ]     [ ]
[S]             [S]     [S]
                [ ]     [ ]


Picosecond 14:
 0   1   2   3   4   5   6
[ ] [S] ... ... ( ) ... [ ]
[ ] [ ]         [ ]     [ ]
[S]             [S]     [S]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [ ] ... ... ( ) ... [ ]
[S] [S]         [ ]     [ ]
[ ]             [ ]     [ ]
                [S]     [S]


Picosecond 15:
 0   1   2   3   4   5   6
[ ] [ ] ... ... [ ] (.) [ ]
[S] [S]         [ ]     [ ]
[ ]             [ ]     [ ]
                [S]     [S]

 0   1   2   3   4   5   6
[S] [S] ... ... [ ] (.) [ ]
[ ] [ ]         [ ]     [ ]
[ ]             [S]     [S]
                [ ]     [ ]


Picosecond 16:
 0   1   2   3   4   5   6
[S] [S] ... ... [ ] ... ( )
[ ] [ ]         [ ]     [ ]
[ ]             [S]     [S]
                [ ]     [ ]

 0   1   2   3   4   5   6
[ ] [ ] ... ... [ ] ... ( )
[S] [S]         [S]     [S]
[ ]             [ ]     [ ]
                [ ]     [ ]
```

Because all smaller delays would get you caught, the fewest number of
picoseconds you would need to delay to get through safely is 10.

What is the fewest number of picoseconds that you need to delay the packet to
pass through the firewall without being caught?
*/

extern crate aoc;

use std::str;

fn main() {
    const MAX_DELAY: usize = 10000000;

    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 13: {}", PUZZLE);

    let firewall = Firewall::new(input.to_str().lines().map(|s| s.parse().unwrap()).collect());
    println!(":: Answer 1 is {}", firewall.traverse(0));

    println!("-> Calculating answer 2 by brute force...");
    let delay = (0..MAX_DELAY).find(|d| !firewall.triggers(*d));
    if let Some(d) = delay {
        println!(":: Answer 2 is {}", d);
    } else {
        println!(":: Answer 2 could not be calculated.");
    }
}

/// A Firewall contains scanners at different levels, each with a range.
///
/// A packet traversing the firewall at time `t` takes
///
#[derive(Debug)]
pub struct Firewall {
    scanners: Vec<Scanner>,
}

impl Firewall {
    pub fn new(vec: Vec<Scanner>) -> Self {
        Self {
            scanners: vec,
        }
    }

    pub fn traverse(&self, time: usize) -> usize {
        self.scanners.iter().fold(0, |acc, s| acc + s.severity(time + s.level))
    }

    pub fn triggers(&self, time: usize) -> bool {
        self.scanners.iter().any(|s| s.is_zero_at(time + s.level))
    }
}

#[derive(Debug)]
pub struct Scanner {
    level: usize,
    range: usize,
}

impl Scanner {
    pub fn new(level: usize, range: usize) -> Self {
        Self {
            level: level,
            range: range,
        }
    }

    pub fn is_zero_at(&self, time: usize) -> bool {
        time % ((self.range - 1) * 2) == 0
    }

    // Returns the severity of being triggered.
    // If not triggered, returns 0.
    pub fn severity(&self, time: usize) -> usize {
        if self.is_zero_at(time) {
            self.level * self.range
        } else {
            0
        }
    }
}

impl str::FromStr for Scanner {
    // TODO: Add a type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: remove the unwrap dammmit
        let list: Vec<usize> = s.split(": ").map(|x| x.parse().unwrap()).collect();
        if list.len() != 2 {
            return Err(());
        }

        Ok(Self{
            level: list[0],
            range: list[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_intensity() {
        let firewall = Firewall::new(
            vec![(0,3), (1,2), (4,4), (6,4)].iter().map(|x| Scanner::new(x.0, x.1)).collect()
        );

        assert_eq!(firewall.traverse(0), 24);
    }
}

const PUZZLE: &'static str = "Packet Scanners";
const INPUT: &'static str = r"
0: 3
1: 2
2: 4
4: 6
6: 4
8: 6
10: 5
12: 8
14: 8
16: 6
18: 8
20: 6
22: 10
24: 8
26: 12
28: 12
30: 8
32: 12
34: 8
36: 14
38: 12
40: 18
42: 12
44: 12
46: 9
48: 14
50: 18
52: 10
54: 14
56: 12
58: 12
60: 14
64: 14
68: 12
70: 17
72: 14
74: 12
76: 14
78: 14
82: 14
84: 14
94: 14
96: 14
";
