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
    println!(":: Answer 2 is {}", disk.regions());
}

#[derive(Clone, Copy, Debug)]
pub struct Bit(isize);

impl Default for Bit {
    fn default() -> Self {
        Bit(0)
    }
}

impl Bit {
    pub const UNSET_CHAR: char = '.';
    pub const SET_CHAR: char = '#';

    pub fn new(set: bool) -> Self {
        Bit(if set { -1 } else { 0 })
    }

    pub fn from_hex(n: u8) -> [Bit; 4] {
        assert!(n.leading_zeros() >= 4);
        let mut array = [Bit::default(); 4];
        for i in 0..4 {
            array[i] = Bit::new(n & (1 << (3 - i)) != 0);
        }
        array
    }

    pub fn unset(&mut self) {
        self.0 = 0;
    }

    pub fn set(&mut self) {
        self.0 = -1;
    }

    pub fn set_region(&mut self, region: usize) {
        assert!(region <= isize::max_value() as usize);
        self.0 = region as isize;
    }

    /// Returns the region of a set bit.
    ///
    /// All unset bits are automatically in region 0.
    pub fn region(&self) -> Option<usize> {
        if self.0 < 0 {
            None
        } else {
            Some(self.0 as usize)
        }
    }

    pub fn is_set(&self) -> bool {
        self.0 != 0
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(match self.region() {
            Some(0) => Bit::UNSET_CHAR,
            Some(n) => ('0' as usize + n % 10) as u8 as char,
            None => Bit::SET_CHAR,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Disk {
    /// Contains a vector of 0-1 strings.
    space: Vec<Vec<Bit>>,
}

impl Disk {
    pub const NUM_BLOCKS: usize = 128;

    pub fn from(s: &str) -> Self {
        // Generate 128 modifications of input.
        Disk {
            space: (0..Disk::NUM_BLOCKS)
                .map(|i| {
                    let hash = knot::hash(format!("{}-{}", s, i).as_str());
                    assert_eq!(hash.len(), 32);
                    hash.chars()
                        .flat_map(|c| Bit::from_hex(c.to_digit(16).unwrap() as u8).to_vec())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn used(&self) -> usize {
        self.space.iter().fold(0, |acc, blk| {
            acc + blk.iter().map(|b| b.is_set() as usize).sum::<usize>()
        })
    }

    pub fn bit(&self, (row, col): (usize, usize)) -> &Bit {
        &self.space[row][col]
    }

    pub fn bit_mut(&mut self, (row, col): (usize, usize)) -> &mut Bit {
        &mut self.space[row][col]
    }

    pub fn regions(&self) -> usize {
        let mut disk = self.clone();
        let mut count = 0;
        for row in 0..Disk::NUM_BLOCKS {
            for col in 0..Disk::NUM_BLOCKS {
                let idx = (row, col);
                if disk.bit(idx).region().is_none() {
                    count += 1;
                    disk.set_region_from(idx, count);
                }
                assert!(disk.bit(idx).region().is_some());
            }
        }
        count
    }

    fn set_region_from(&mut self, idx: (usize, usize), region: usize) {
        let mut stack = vec![idx];
        loop {
            match stack.pop() {
                None => break,
                Some(idx) => {
                    let b = self.bit_mut(idx);
                    if b.region().is_none() {
                        b.set_region(region);
                        stack.append(&mut neighbours(Disk::NUM_BLOCKS, idx));
                    }
                }
            }
        }
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.space.iter() {
            line.iter().for_each(|b| write!(f, "{}", b).unwrap());
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn neighbours_wrapping(block_size: usize, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        ((row - 1) % block_size, col),
        ((row + 1) % block_size, col),
        (row, (col - 1) % block_size),
        (row, (col + 1) % block_size),
    ]
}

fn neighbours(block_size: usize, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let max_idx = block_size - 1;
    let mut vec = Vec::new();
    if row > 0 {
        vec.push((row - 1, col));
    }
    if row < max_idx {
        vec.push((row + 1, col));
    }
    if col > 0 {
        vec.push((row, col - 1));
    }
    if col < max_idx {
        vec.push((row, col + 1));
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_used_blocks() {
        let disk = Disk::from("flqrgnkx");
        assert_eq!(disk.used(), 8108);
    }

    #[test]
    fn test_regions() {
        let mut disk = Disk::from("flqrgnkx");
        let regions = disk.regions();
        println!("\n{}", disk);
        assert_eq!(regions, 1242);
    }
}

const PUZZLE: &'static str = "Disk Defragmentation";
const INPUT: &'static str = "ffayrhll";
