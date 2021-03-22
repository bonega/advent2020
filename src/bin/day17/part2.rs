use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::RangeInclusive;

pub(crate) fn main() {
    let s = include_str!("input.txt");
    let mut d = Dimension::new(s);
    d.tick();
    d.tick();
    d.tick();
    d.tick();
    d.tick();
    d.tick();
    println!("Problem2: {}", d.active_cubes.len());
}

type Index = (isize, isize, isize, isize);

struct Dimension {
    active_cubes: HashSet<Index>
}

struct DimensionSlice<'a> {
    z: isize,
    w: isize,
    parent_dimension: &'a Dimension,
}

impl<'a> DimensionSlice<'a> {
    fn frame(&self) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
        let mut min_x = isize::MAX;
        let mut max_x = 0;
        let mut min_y = isize::MAX;
        let mut max_y = 0;
        for (x, y, ..) in &self.parent_dimension.active_cubes {
            if x < &min_x {
                min_x = *x;
            }

            if x > &max_x {
                max_x = *x;
            }

            if y < &min_y {
                min_y = *y;
            }

            if y > &max_y {
                max_y = *y;
            }
        }
        (min_x..=max_x, min_y..=max_y)
    }
}

impl<'a> Display for DimensionSlice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (rows, cols) = self.frame();
        write!(f, "z={}", self.z)?;
        for y in cols {
            write!(f, "\n")?;
            for x in rows.clone() {
                let index = (x, y, self.z, self.w);
                let c = match self.parent_dimension.active_cubes.contains(&index) {
                    true => '#',
                    false => '.',
                };
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

impl Dimension {
    fn new(s: &str) -> Self {
        let mut cubes = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => { cubes.insert((x as isize, y as isize, 0, 0)); }
                    '.' => {}
                    _ => unreachable!(c),
                };
            }
        }
        Dimension { active_cubes: cubes }
    }

    fn slice(&self, z: isize, w: isize) -> DimensionSlice {
        DimensionSlice { z, w, parent_dimension: self }
    }

    fn neighbors(&self, index: &Index) -> Vec<Index> {
        let (x0, y0, z0, w0) = index;
        let mut neighbors = Vec::new();
        for x in x0 - 1..=x0 + 1 {
            for y in y0 - 1..=y0 + 1 {
                for z in z0 - 1..=z0 + 1 {
                    for w in w0 - 1..=w0 + 1 {
                        let current_index = (x, y, z, w);
                        if *index != current_index {
                            neighbors.push(current_index);
                        }
                    }
                }
            }
        }
        neighbors
    }

    fn tick(&mut self) {
        let mut remove_list = Vec::new();
        let mut inactive_neighbors = HashMap::new();
        for index in &self.active_cubes {
            let neighbors = self.neighbors(&index);
            let active_neighbors = neighbors.iter().filter(|index| self.active_cubes.contains(index));
            if !(2..=3).contains(&active_neighbors.count()) {
                remove_list.push(*index);
            }
            neighbors.iter()
                .filter(|index| !self.active_cubes.contains(index))
                .for_each(|index| {
                    *inactive_neighbors.entry(*index).or_insert(0) += 1
                })
        }
        inactive_neighbors.iter()
            .filter(|(_, n)| **n == 3)
            .for_each(|(&index, _)| {
                self.active_cubes.insert(index);
            });
        remove_list.iter().for_each(|index| {
            self.active_cubes.remove(index);
        });
    }
}

#[test]
fn parse_dimension_test() {
    let s = ".#.
..#
###";
    let dimension = Dimension::new(s);
    let slice = dimension.slice(0, 0);
    let res = format!("{}", slice);
    assert_eq!("z=0\n".to_owned() + s, res);
}

#[test]
fn test_six_cycles() {
    let s = ".#.
..#
###";
    let mut d = Dimension::new(s);
    d.tick();
    d.tick();
    d.tick();
    d.tick();
    d.tick();
    d.tick();
    assert_eq!(848, d.active_cubes.len());
}
