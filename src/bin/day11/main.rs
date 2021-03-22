use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::ops::Index;

use Square::Seat;

use crate::square::{SeatState::{Empty, Occupied}, Square};

mod square;
mod part2;

struct Map {
    squares: Vec<Square>,
    rows: usize,
    cols: usize,
}


impl Map {
    fn index_from(&self, x: usize, y: usize) -> usize {
        self.cols * y + x
    }

    fn neighbors(&self, index: usize) -> Vec<Square> {
        let x = (index % self.cols) as isize;
        let y = (index / self.cols) as isize;
        let indices = [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
            (x - 1, y), (x + 1, y),
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
        indices.iter()
            .filter(|(x, y)| *x >= 0 && x < &(self.cols as isize) && *y >= 0 && y < &(self.rows as isize))
            .map(|(x, y)| self.squares[self.index_from(*x as usize, *y as usize)])
            .collect()
    }

    fn tick(&mut self) -> bool {
        use Square::*;
        let updates: Vec<_> = self.squares.iter()
            .enumerate()
            .filter_map(|(i, square)| {
                let full_neighbors = self.neighbors(i).iter()
                    .filter(|x| **x == Seat(Occupied)).count();
                match (square, full_neighbors) {
                    (Seat(Empty), 0) => Some((i, Seat(Occupied))),
                    (Seat(Occupied), x) if x >= 4 => Some((i, Seat(Empty))),
                    _ => None
                }
            }).collect();

        let changed = !updates.is_empty();
        for (i, square) in updates.into_iter() {
            self.squares[i] = square;
        }
        changed
    }

    fn run(&mut self) {
        while self.tick() {};
    }

    fn nr_occupied_seats(&self) -> usize {
        self.squares.iter().filter(|x| **x == Seat(Occupied)).count()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, square) in self.squares.iter().enumerate() {
            if i % self.cols == 0 && i != 0 {
                f.write_char('\n')?;
            }
            write!(f, "{}", square)?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Square;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.squares[self.cols * y + x]
    }
}


impl Map {
    fn new(s: &str) -> Self {
        let cols = s.find('\n').unwrap();
        let squares: Vec<_> = s.lines()
            .flat_map(str::chars)
            .map(Square::new).collect();
        let rows = squares.len() / cols;
        Map { squares, rows, cols }
    }
}

#[cfg(test)]
mod tests {
    use Square::*;

    use super::*;

    const SIMPLE_TEST_DATA: &str =
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_from() {
        let m = Map::new(SIMPLE_TEST_DATA);
        assert_eq!(10, m.rows);
        assert_eq!(10, m.cols);
        assert_eq!(11, m.index_from(1, 1));
        assert_eq!(Seat(Empty), m[(0, 0)]);
        assert_eq!(Floor, m[(1, 2)]);
        assert_eq!(Seat(Empty), m[(9, 9)]);
    }

    #[test]
    fn test_neighbors() {
        let m = Map::new(SIMPLE_TEST_DATA);
        assert_eq!(vec![Floor, Seat(Empty), Seat(Empty)], m.neighbors(0));
    }

    #[test]
    fn simple_stable() {
        let mut m = Map::new(SIMPLE_TEST_DATA);
        m.run();
        assert_eq!(37, m.nr_occupied_seats());
    }
}

fn main() {
    let s = include_str!("input.txt");
    let mut m = Map::new(s);
    m.run();
    println!("Problem1 {}", m.nr_occupied_seats());
    part2::main();
}
