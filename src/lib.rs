// Calculate the captcha result for the printer for day 1.
// This works for part one (n=1) and part two (n=vec.len()/2).
pub fn inverse_captcha<'a, I>(it: I, n: usize) -> u32
where I: Iterator<Item = &'a u32> + std::clone::Clone {
    let y = it.clone().cycle().skip(n);
    it.zip(y).fold(0, |acc, (x,y)| if x == y { acc + x } else { acc })
}

// Calculate the checksum of the spreadsheet.
pub fn spreadsheet_checksum<'a, I>(it: I) -> u32
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
pub fn spreadsheet_checksum_div<'a, I>(it: I) -> u32
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
            assert_eq!(spreadsheet_checksum(t.0.iter()), t.1);
            assert_eq!(spreadsheet_checksum_div(t.0.iter()), t.2);
        }
    }

    #[test]
    fn inverse_captcha_correct_sum() {
        let tests = vec![
            // vector       1  n/2
            (vec![1,1,2,2], 3, 0),
            (vec![1,2,1,2], 0, 6),
            (vec![1,1,1,1], 4, 4),
            (vec![1,2,3,4], 0, 0),
            (vec![1,1,1,2], 2, 2),
            (vec![9,1,2,1,2,1,2,9], 9, 6),
        ];

        for t in tests {
            assert_eq!(inverse_captcha(t.0.iter(), 1), t.1);
            assert_eq!(inverse_captcha(t.0.iter(), t.0.len()/2), t.2);
        }
    }
}
