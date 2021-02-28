use std::fmt::{Display, Formatter, Write};
use std::{iter, fmt};
use std::ops::Index;

use crate::square::{SeatState::{Empty, Occupied}, Square};
use Square::Seat;

#[derive(Copy, Clone, Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

struct Neighbors<'a> {
    map: &'a Map,
    direction: Option<Direction>,
    origin: (usize, usize),
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;

        if self.direction.is_none() {
            return None;
        }
        let (dx, dy) = match self.direction.unwrap() {
            N => (0, 1),
            NE => (1, 1),
            E => (1, 0),
            SE => (1, -1),
            S => (0, -1),
            SW => (-1, -1),
            W => (-1, 0),
            NW => (-1, 1),
        };
        let max_x = match dx {
            1 => Some(self.map.cols - 1 - self.origin.0),
            -1 => Some(self.origin.0),
            _ => None,
        };
        let max_y = match dy {
            1 => Some(self.map.rows - 1 - self.origin.1),
            -1 => Some(self.origin.1),
            _ => None,
        };
        let n: usize = *[max_x, max_y]
            .iter()
            .flatten()
            .min()
            .unwrap_or(&0);
        let res = iter::successors(Some(self.origin), |(x, y)| Some((x.wrapping_add(dx as usize), y.wrapping_add(dy as usize))))
            .skip(1)
            .take(n)
            .map(|index| self.map[index])
            .find(Square::is_seat);
        self.direction = self.direction.and_then(|x| match x {
            N => Some(NE),
            NE => Some(E),
            E => Some(SE),
            SE => Some(S),
            S => Some(SW),
            SW => Some(W),
            W => Some(NW),
            NW => None,
        });
        if res.is_some() {
            res
        } else {
            self.next()
        }
    }
}


struct Map {
    squares: Vec<Square>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn index_from(&self, x: usize, y: usize) -> usize {
        self.cols * y + x
    }

    fn neighbors_iter(&self, index: usize) -> Neighbors {
        let x = index % self.cols;
        let y = index / self.cols;
        Neighbors {
            map: &self,
            direction: Some(Direction::N),
            origin: (x, y),
        }
    }

    fn tick(&mut self) -> bool {
        use Square::*;
        let updates: Vec<_> = self.squares.iter()
            .enumerate()
            .filter_map(|(i, square)| {
                let full_neighbors = self.neighbors_iter(i)
                    .filter(|x| *x == Seat(Occupied)).count();
                match (square, full_neighbors) {
                    (Seat(Empty), 0) => Some((i, Seat(Occupied))),
                    (Seat(Occupied), x) if x >= 5 => Some((i, Seat(Empty))),
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


impl From<&str> for Map {
    fn from(s: &str) -> Self {
        use Square::*;
        let cols = s.find('\n').unwrap();
        let squares: Vec<_> = s.lines().flat_map(|x| x.chars()
            .map(|c| {
                match c {
                    '#' => Seat(Occupied),
                    'L' => Seat(Empty),
                    _ => Floor,
                }
            })).collect();
        let rows = squares.len() / cols;
        Map { squares, rows, cols }
    }
}


#[cfg(test)]
mod tests {
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
    fn test_simple_stable() {
        let mut m = Map::from(SIMPLE_TEST_DATA);
        m.run();
        assert_eq!(26, m.nr_occupied_seats());
    }

    #[test]
    fn test_simple_neighbors() {
        let m = Map::from(SIMPLE_TEST_DATA);
        let i = m.index_from(2, 0);
        let neighbors = m.neighbors_iter(i);
        assert_eq!(5, neighbors.count());
    }
}

pub(crate) fn main() {
    let s = include_str!("input.txt");
    let mut m = Map::from(s);
    m.run();
    println!("Problem2 {}", m.nr_occupied_seats());
}
