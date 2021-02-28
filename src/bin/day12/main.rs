use Direction::{East, North, South, West};
mod part2;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    fn right(&self) -> Self {
        use Direction::*;
        match self {
            West => North,
            East => South,
            North => East,
            South => West,
        }
    }
}


struct Ferry {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Ferry {
    fn new() -> Self {
        Self { x: 0, y: 0, dir: Direction::East }
    }

    fn turn(&mut self, deg: isize) {
        let times = (deg / 90).abs();
        let turn_dir = match deg.is_positive() {
            false => Direction::left,
            true => Direction::right,
        };

        for _ in 0..times {
            self.dir = turn_dir(&self.dir);
        }
    }

    fn move_dir(&mut self, direction: &Direction, times: isize) {
        match direction {
            Direction::North => self.y -= times,
            Direction::South => self.y += times,
            Direction::East => self.x += times,
            Direction::West => self.x -= times,
        }
    }

    #[allow(dead_code)]
    fn execute(&mut self, s: &str) {
        let arg: isize = s[1..].parse().unwrap();
        match s.chars().nth(0).unwrap() {
            'L' => self.turn(-arg),
            'R' => self.turn(arg),
            s => {
                let dir = match s {
                    'F' => self.dir,
                    'N' => North,
                    'E' => East,
                    'S' => South,
                    'W' => West,
                    _ => unreachable!(),
                };
                self.move_dir(&dir, arg)
            }
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::Ferry;
    const SIMPLE_DATA:&str="F10
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
        assert_eq!(17, ferry.x);
        assert_eq!(8, ferry.y);
        assert_eq!(25, ferry.manhattan_distance());
    }
}

fn main() {
    let mut ferry = Ferry::new();
    let s = include_str!("input.txt");
    for line in s.lines() {
        ferry.execute(line);
    }
    println!("Problem1: {}", ferry.manhattan_distance());
    part2::main();
}
