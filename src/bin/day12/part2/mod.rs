use std::fmt::{Display, Formatter};
use std::fmt;

struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    fn rotate(&mut self, angle: f64) {
        let times = angle.abs() as isize / 90;
        if angle.is_sign_positive() {
            for _ in 0..times {
                let x0 = self.x;
                self.x = -self.y;
                self.y = x0;
            }
        } else {
            for _ in 0..times {
                let x0 = self.x;
                self.x = self.y;
                self.y = -x0;
            }
        }
    }
}

struct Ferry {
    x: isize,
    y: isize,
    waypoint: Waypoint,
}

impl Display for Ferry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Ferry x: {}, y {}", self.x, self.y)
    }
}

impl Ferry {
    fn new() -> Self {
        Self { x: 0, y: 0, waypoint: Waypoint { x: 10, y: -1 } }
    }


    #[allow(dead_code)]
    fn execute(&mut self, s: &str) {
        let arg: isize = s[1..].parse().unwrap();
        match s.chars().nth(0).unwrap() {
            'F' => {
                self.x += self.waypoint.x * arg;
                self.y += self.waypoint.y * arg;
            }
            'N' => self.waypoint.y -= arg,
            'E' => self.waypoint.x += arg,
            'S' => self.waypoint.y += arg,
            'W' => self.waypoint.x -= arg,
            'L' => self.waypoint.rotate(-arg as f64),
            'R' => self.waypoint.rotate(arg as f64),
            _ => unreachable!(),
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::part2::Ferry;

    const SIMPLE_DATA: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_simple() {
        let mut ferry = Ferry::new();
        for line in SIMPLE_DATA.lines() {
            ferry.execute(line);
        }
        assert_eq!(214, ferry.x);
        assert_eq!(72, ferry.y);
        assert_eq!(286, ferry.manhattan_distance());
    }

    #[test]
    fn test_wp_rotation() {
        let mut ferry = Ferry::new();
        ferry.waypoint.rotate(90.0);
        assert_eq!(1, ferry.waypoint.x);
        assert_eq!(10, ferry.waypoint.y);
    }
}

pub fn solve(s: &str) -> usize {
    let mut ferry = Ferry::new();
    for line in s.lines() {
        ferry.execute(line);
    }
    ferry.manhattan_distance()
}
