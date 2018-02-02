/*!
# Day 14: Disk Defragmentation

Suddenly, a scheduled job activates the system's disk defragmenter. Were the
situation different, you might sit and watch it for a while, but today, you just
don't have that kind of time. It's soaking up valuable system resources that are
needed elsewhere, and so the only option is to help it finish its task as soon
as possible.

The disk in question consists of a 128x128 grid; each square of the grid is
either free or used. On this disk, the state of the grid is tracked by the bits
in a sequence of knot hashes.

A total of 128 knot hashes are calculated, each corresponding to a single row in
the grid; each hash contains 128 bits which correspond to individual grid
squares. Each bit of a hash indicates whether that square is free (0) or used
(1).

The hash inputs are a key string (your puzzle input), a dash, and a number from
0 to 127 corresponding to the row. For example, if your key string were
`flqrgnkx`, then the first row would be given by the bits of the knot hash of
`flqrgnkx-0`, the second row from the bits of the knot hash of `flqrgnkx-1`, and so
on until the last row, `flqrgnkx-127`.

The output of a knot hash is traditionally represented by 32 hexadecimal digits;
each of these digits correspond to 4 bits, for a total of `4 * 32 = 128` bits.
To convert to bits, turn each hexadecimal digit to its equivalent binary value,
high-bit first: 0 becomes `0000`, 1 becomes `0001`, e becomes `1110`, f becomes
`1111`, and so on; a hash that begins with `a0c2017...` in hexadecimal would
begin with `10100000110000100000000101110000...` in binary.

Continuing this process, the first 8 rows and columns for key `flqrgnkx` appear as
follows, using `#` to denote used squares, and `.` to denote free ones:

```text
##.#.#..-->
.#.#.#.#
....#.#.
#.#.##.#
.##.#...
##..#..#
.#...#..
##.#.##.-->
|      |
V      V
```

In this example, 8108 squares are used across the entire 128x128 grid.

Given your actual key string, how many squares are used?

## Part Two

Now, all the defragmenter needs to know is the number of regions. A region is
a group of used squares that are all adjacent, not including diagonals. Every
used square is in exactly one region: lone used squares form their own isolated
regions, while several adjacent squares all count as a single region.

In the example above, the following nine regions are visible, each marked with
a distinct digit:

```text
11.2.3..-->
.1.2.3.4
....5.6.
7.8.55.9
.88.5...
88..5..8
.8...8..
88.8.88.-->
|      |
V      V
```

Of particular interest is the region marked 8; while it does not appear
contiguous in this small view, all of the squares marked 8 are connected when
considering the whole 128x128 grid. In total, in this example, 1242 regions are
present.

How many regions are present given your key string?
*/

extern crate aoc;

use aoc::knot;
use std::fmt::{self, Write};

#[allow(unused)]
fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 14: {}", PUZZLE);

    let disk = Disk::from(input.to_str());
    println!(":: Answer 1 is {}", disk.used());
}

pub struct Bit {
    set: bool,
}

impl Bit {
    pub const UNSET_CHAR: char = '.';
    pub const SET_CHAR: char = '#';

    fn new(set: bool) -> Self {
        Self {
            set: set,
        }
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(if self.set { Bit::SET_CHAR } else { Bit::UNSET_CHAR })
    }
}

pub struct Disk {
    /// Contains a vector of 0-1 strings.
    space: Vec<Vec<bool>>
}

impl Disk {
    pub const NUM_BLOCKS: usize = 128;

    pub fn from(s: &str) -> Self {
        // Generate 128 modifications of input.
        Disk {
            space: (0..Self::NUM_BLOCKS)
                .map(|i| {
                    let hash = knot::hash(format!("{}-{}", s, i).as_str());
                    hash.chars()
                        .map(|c| c.to_digit(16).unwrap() as u8)
                        // needs to be split into 8 bools
                        .collect()
                })
                .collect()
        }
    }

    pub fn used(&self) -> usize {
        self.space.iter().fold(0, |acc, blk| {
            acc + blk.iter().fold(0, |acc, b| acc + if *b { 1 } else { 0 })
        })
    }

    pub fn regions(&self) -> usize {
        // Not sure how I want to go about this.
        let mut blocks: Vec<_> = format!("{}", self).lines().map(|x| String::from(x)).collect();
        for li in 0..Disk::NUM_BLOCKS {
            
        }

        0
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.space.iter() {
            let mut s = String::with_capacity(Disk::NUM_BLOCKS);
            line.iter().for_each(|b| write!(f, "{}", b).unwrap());
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_used_blocks() {
        let test = ("flqrgnkx", 8108);
        assert_eq!(Disk::from(test.0).used(), 8108);
        assert_eq!(Disk::regions(), 1242);
    }
}

const PUZZLE: &'static str = "Disk Defragmentation";
const INPUT: &'static str = "ffayrhll";
