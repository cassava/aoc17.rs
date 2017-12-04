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
