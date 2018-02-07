/*!
# Day 17: Spinlock ---

Suddenly, whirling in the distance, you notice what looks like a massive,
pixelated hurricane: a deadly spinlock. This spinlock isn't just consuming
computing power, but memory, too; vast, digital mountains are being ripped from
the ground and consumed by the vortex.

If you don't move quickly, fixing that printer will be the least of your problems.

This spinlock's algorithm is simple but efficient, quickly consuming everything
in its path. It starts with a circular buffer containing only the value 0, which
it marks as the current position. It then steps forward through the circular
buffer some number of steps (your puzzle input) before inserting the first new
value, 1, after the value it stopped on. The inserted value becomes the current
position. Then, it steps forward from there the same number of steps, and
wherever it stops, inserts after it the second new value, 2, and uses that as
the new current position again.

It repeats this process of stepping forward, inserting a new value, and using
the location of the inserted value as the new current position a total of 2017
times, inserting 2017 as its final operation, and ending with a total of 2018
values (including 0) in the circular buffer.

For example, if the spinlock were to step 3 times per insert, the circular
buffer would begin to evolve like this (using parentheses to mark the current
position after each iteration of the algorithm):

- `(0)`, the initial state before any insertions.
- `0 (1)`: the spinlock steps forward three times `(0, 0, 0)`, and then inserts
  the first value, 1, after it. 1 becomes the current position.
- `0 (2) 1`: the spinlock steps forward three times `(0, 1, 0)`, and then
  inserts the second value, 2, after it. 2 becomes the current position.
- `0  2 (3) 1`: the spinlock steps forward three times `(1, 0, 2)`, and then
  inserts the third value, 3, after it. 3 becomes the current position.

And so on:

```text
0  2 (4) 3  1
0 (5) 2  4  3  1
0  5  2  4  3 (6) 1
0  5 (7) 2  4  3  6  1
0  5  7  2  4  3 (8) 6  1
0 (9) 5  7  2  4  3  8  6  1
```

Eventually, after 2017 insertions, the section of the circular buffer near the
last insertion looks like this:

```text
1512  1134  151 (2017) 638  1513  851
```

Perhaps, if you can identify the value that will ultimately be after the last
value written `(2017)`, you can short-circuit the spinlock. In this example, that
would be 638.

What is the value after 2017 in your completed circular buffer?

## Part Two

The spinlock does not short-circuit. Instead, it gets more angry. At least, you
assume that's what happened; it's spinning significantly faster than it was
a moment ago.

You have good news and bad news.

The good news is that you have improved calculations for how to stop the
spinlock. They indicate that you actually need to identify the value after 0 in
the current state of the circular buffer.

The bad news is that while you were determining this, the spinlock has just
finished inserting its fifty millionth value (50000000).

What is the value after 0 the moment 50000000 is inserted?
*/

extern crate aoc;

fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 17: {}", PUZZLE);

    let steps: usize = input.to_str().parse().unwrap();

    println!(":: Answer 1 is {}", whirlwind_next(steps, 2017));
    println!(
        ":: Answer 2 is {}",
        whirlwind(steps, 50_000_000, 1).watch_value.unwrap()
    );
}

#[derive(Debug, PartialEq, PartialOrd)]
struct WhirlMatch {
    pub last_position: usize,
    pub watch_set_at: usize,
    pub watch_value: Option<usize>,
}

fn whirlwind(steps: usize, until: usize, watch: usize) -> WhirlMatch {
    let mut len = 1;
    let mut pos = 0;
    let mut val = if watch == 0 { Some(0) } else { None };

    // The value at 0 is always the same, we can skip the watching.
    if watch == 0 {
        for _ in 1..(until + 1) {
            pos = ((pos + steps) % len) + 1;
            len += 1;
        }
        return WhirlMatch {
            last_position: pos,
            watch_set_at: 0,
            watch_value: Some(0),
        };
    }

    // If the watch index is higher than 1, we need to assume that it could
    // have been pushed there at a later time. So we need to keep track
    // how many times the value sitting at i was pushed to the left, that

    let mut slider = watch;
    for i in 1..(until + 1) {
        pos = ((pos + steps) % len) + 1;
        len += 1;

        // Update the slider
        if pos > watch {
            ;
        } else if pos == watch {
            val = Some(i);
            slider = pos;
        } else if pos < slider {
            slider -= 1;
        } else if slider < pos {
            slider += pos - slider;
        }
        assert!(slider <= watch);
    }

    // Do the whole thing again, now watch the value of slider though.
    len = 1;
    pos = 0;
    val = if slider == 0 { Some(0) } else { None };
    for i in 1..(until + 1) {
        pos = ((pos + steps) % len) + 1;
        len += 1;

        // Update the value
        if pos == slider {
            val = Some(i);
        }
    }

    WhirlMatch {
        last_position: pos,
        watch_set_at: slider,
        watch_value: val,
    }
}

fn whirlwind_next(steps: usize, until: usize) -> usize {
    // First, find out what the last index is, then use that.
    let WhirlMatch {
        last_position: pos, ..
    } = whirlwind(steps, until, 0);

    let watch = pos + 1;
    if watch % (until + 1) == 0 {
        0
    } else {
        whirlwind(steps, until, watch).watch_value.unwrap()
    }
}

#[test]
fn test_whirlwind_zerostep() {
    for until in 0..100 {
        for watch in 0..until {
            assert_eq!(
                whirlwind(0, until, watch),
                WhirlMatch {
                    last_position: until,
                    watch_set_at: watch,
                    watch_value: Some(watch),
                }
            );
        }
    }
}

#[test]
fn test_whirlwind_next_zerostep() {
    for until in 0..1000 {
        assert_eq!(whirlwind_next(0, until), 0);
    }
}

#[test]
fn test_whirlwind_basics() {
    let tests: Vec<(usize, usize, Vec<usize>, usize)> = vec![
        (3, 0, vec![0], 0),
        (3, 1, vec![0, 1], 1),
        (3, 2, vec![0, 2, 1], 1),
        (3, 3, vec![0, 2, 3, 1], 2),
        (3, 4, vec![0, 2, 4, 3, 1], 2),
        (3, 5, vec![0, 5, 2, 4, 3, 1], 1),
        (3, 6, vec![0, 5, 2, 4, 3, 6, 1], 5),
        (3, 7, vec![0, 5, 7, 2, 4, 3, 6, 1], 2),
        (3, 8, vec![0, 5, 7, 2, 4, 3, 8, 6, 1], 6),
        (3, 9, vec![0, 9, 5, 7, 2, 4, 3, 8, 6, 1], 1),
    ];
    for (steps, until, results, pos) in tests {
        let n = results.len();
        assert_eq!(until + 1, n);
        for index in 0..n {
            let mat = whirlwind(steps, until, index);
            assert_eq!(mat.last_position, pos);
            assert_eq!(
                mat.watch_value,
                Some(results[index]),
                "watch index {} with test {:?} => {:?}",
                index,
                results,
                mat,
            );
        }
        assert_eq!(
            whirlwind_next(steps, until),
            results[(pos + 1) % (until + 1)]
        );
    }
}

const PUZZLE: &'static str = "Spinlock";
const INPUT: &'static str = "371";
