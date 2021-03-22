use core::fmt::{Display, Formatter};
use core::fmt;
use std::fmt::Write;

use SeatState::{Empty, Occupied};
use Square::{Floor, Seat};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SeatState {
    Occupied,
    Empty,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Square {
    Floor,
    Seat(SeatState),
}

impl Square {
    pub fn new(c: char) -> Self {
        match c {
            '#' => Seat(Occupied),
            'L' => Seat(Empty),
            _ => Floor,
        }
    }

    pub fn is_seat(&self) -> bool {
        match self {
            Seat(_) => true,
            _ => false,
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Floor => '.',
            Seat(Occupied) => '#',
            Seat(Empty) => 'L',
        };
        f.write_char(c)
    }
}
