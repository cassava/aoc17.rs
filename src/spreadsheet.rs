use std::io;
use std::io::prelude::*;

pub fn read_from_stdin() -> Vec<Vec<u32>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let v = lines.map(|s| s.as_ref().unwrap()
                      .split_whitespace()
                      .map(|x| x.parse::<u32>().unwrap())
                      .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    v
}

// Calculate the checksum of the spreadsheet.
pub fn checksum<'a, I>(it: I) -> u32
where I: Iterator<Item = &'a Vec<u32>> {
    it.fold(0, |acc, v| {
        let mut it = v.iter();
        if let Some(x) = it.next() {
            let mut min = x;
            let mut max = x;
            for y in it {
                if y > max {
                    max = y
                }
                if y < min {
                    min = y
                }
            }
            acc + (max - min)
        } else {
            acc
        }
    })
}

// Calculate the checksum of the spreadsheet,
// using the second part of the algorithm.
//
// For each row, the checksum is calculated by
// dividing the largest number that can *cleanly*
// be divided by the smallest number.
//
// Note that this is *not* what is required by the second
// challenge, but it is safer as it makes less assumptions.
pub fn checksum_div<'a, I>(it: I) -> u32
where I: Iterator<Item = &'a Vec<u32>> {
    it.fold(0, |acc, v| {
        let mut max = 0;
        for x in v.iter() {
            for y in v.iter() {
                if x % y != 0 {
                    continue;
                } else if max < (x / y) {
                    max = x / y;
                }
            }
        }
        acc + max
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spreadsheet_checksum_correct_sum() {
        let tests = vec![
            // vector       max-min  div
            (vec![vec![5,1,9,5]], 8, 9),
            (vec![vec![5,2,9,8]], 7, 4),
            (vec![vec![7,5,3]], 4, 1),
            (vec![vec![5,1,9,5], vec![7,5,3], vec![2,4,6,8]], 18, 14),
        ];

        for t in tests {
            assert_eq!(checksum(t.0.iter()), t.1);
            assert_eq!(checksum_div(t.0.iter()), t.2);
        }
    }
}
