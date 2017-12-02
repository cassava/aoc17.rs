// Calculate the captcha result for the printer for day 1.
// This works for part one (n=1) and part two (n=vec.len()/2).
pub fn inverse_captcha<'a, I>(it: I, n: usize) -> u32
where I: Iterator<Item = &'a u32> + std::clone::Clone {
    let y = it.clone().cycle().skip(n);
    it.zip(y).fold(0, |acc, (x,y)| if x == y { acc + x } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_captcha_correct_sum() {
        let tests = vec![
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
