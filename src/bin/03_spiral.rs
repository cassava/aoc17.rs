/*!
# Day 3: Spiral Memory

You come across an experimental new kind of memory stored on an infinite two-dimensional grid.

Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and then
counting up while spiraling outward. For example, the first few squares are allocated like this:

```text
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...
```

While this is very space-efficient (no squares are skipped), requested data must be carried back to
square 1 (the location of the only access port for this memory system) by programs that can only
move up, down, left, or right. They always take the shortest path: the Manhattan Distance between
the location of the data and square 1.

For example:

- Data from square 1 is carried 0 steps, since it's at the access port.
- Data from square 12 is carried 3 steps, such as: down, left, left.
- Data from square 23 is carried only 2 steps: up twice.
- Data from square 1024 must be carried 31 steps.

How many steps are required to carry the data from the square identified in your puzzle input all
the way to the access port?

## Part Two

As a stress test on the system, the programs here clear the grid and then store the value 1 in
square 1. Then, in the same allocation order as shown above, they store the sum of the values in
all adjacent squares, including diagonals.

So, the first few squares' values are chosen as follows:

- Square 1 starts with the value 1.
- Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
- Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
- Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
- Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.

Once a square is written, its value does not change. Therefore, the first few squares would receive
the following values:

```text
147  142  133  122   59
304    5    4    2   57
330   10    1    1   54
351   11   23   25   26
362  747  806--->   ...
```

What is the first value written that is larger than your puzzle input?
*/

extern crate aoc;

use std::collections::HashMap;

fn main() {
    let mut input = aoc::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 3: {}", PUZZLE);

    let n = input.to_str().parse::<u32>().unwrap();

    println!(":: Answer 1 is {}", Point::new(n - 1).manhattan_distance_from_origin());

    // Perform the stress test.
    // TODO: this is super-inefficient!
    let mut i = 1;
    let mut memory = HashMap::new();
    memory.insert(Point::new(0), 1);
    let sum = loop {
        let p = Point::new(i);
        let sum = p.neighbours().iter().fold(0, |acc, x| if let Some(v) = memory.get(x) { acc + v } else { acc });
        memory.insert(p, sum);
        if sum > n {
            break sum;
        }
        i += 1;
    };
    println!(":: Answer 2 is {}", sum);
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point{
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(n: u32) -> Self {
        let mut p = Point{x: 0, y: 0};

        // Calculating the point in euclidean space is not really
        // that complicated, but this algorithm looks the part.
        let mut repeat = 1;
        let mut current = (1, 1);
        let mut direction = 0;
        let mut n = n;
        while n > 0 {
            n -= 1;

            // Go a direction
            match direction {
                0 => p.x += 1,
                1 => p.y += 1,
                2 => p.x -= 1,
                3 => p.y -= 1,
                _ => panic!("logic error"),
            }

            // Update the state
            repeat -= 1;
            if repeat == 0 {
                // Goes from (1, 1) -> (1, 2) -> (2, 2), (2, 3), ...
                if current.0 == current.1 {
                    current.1 += 1;
                } else {
                    current.0 += 1;
                }
                repeat = current.0;
                direction = (direction + 1) % 4;
            }
        }

        p
    }

    pub fn manhattan_distance_from_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    // TODO: Turn this into an iterator!
    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            Point{x: self.x + 1, y: self.y + 0},
            Point{x: self.x + 1, y: self.y + 1},
            Point{x: self.x + 0, y: self.y + 1},
            Point{x: self.x - 1, y: self.y + 1},
            Point{x: self.x - 1, y: self.y + 0},
            Point{x: self.x - 1, y: self.y - 1},
            Point{x: self.x + 0, y: self.y - 1},
            Point{x: self.x + 1, y: self.y - 1},
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiral_point_correct() {
        let tests = vec![
            (0, Point{x: 0, y: 0}),
            (1, Point{x: 1, y: 0}),
            (2, Point{x: 1, y: 1}),
        ];

        for t in tests {
            assert_eq!(Point::new(t.0), t.1)
        }
    }

    #[test]
    fn spiral_distance_correct() {
        let tests = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (11, 3),
            (22, 2),
            (1023, 31),
        ];

        for t in tests {
            assert_eq!(Point::new(t.0).manhattan_distance_from_origin(), t.1);
        }
    }
}

const PUZZLE: &'static str = "Spiral Memory";
const INPUT: &'static str = r"
289326
";
