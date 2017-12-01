extern crate num;

pub fn inverse_captcha<I: IntoIterator>(it: I) -> I::Item
where I::Item: num::Integer + num::Zero + Copy
{
    let mut sum = num::zero();
    let mut it = it.into_iter();

    let first;
    let mut last;
    match it.next() {
        Some(i) => {
            first = i;
            last = i;
            },
        None => {return sum; }
    }

    while let Some(i) = it.next() {
        if last == i {
            sum = sum + last
        }
        last = i;
    }

    if first == last {
        sum + first
    } else {
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_captcha_correct_sum() {
        let tests = vec![
            (vec![1,1,2,2], 3),
            (vec![1,1,1,1], 4),
            (vec![1,2,3,4], 0),
            (vec![1,1,1,2], 2),
            (vec![9,1,2,1,2,1,2,9], 9),
        ];

        for t in tests {
            assert_eq!(inverse_captcha(t.0), t.1);
        }
    }
}
