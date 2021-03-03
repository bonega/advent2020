use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("input.txt");

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let groups = INPUT.split_terminator("\n\n");
    let res: usize = groups.map(|group| {
        group.lines()
            .flat_map(|x| x.chars())
            .collect::<HashSet<char>>().len()
    }).sum();
    println!("Problem1 {}", res)
}

fn problem2() {
    let groups = INPUT.split_terminator("\n\n");
    let res: usize = groups.map(|group| {
        let mut lines = group.lines()
            .map(|x| HashSet::from_iter(x.chars()));
        let init: HashSet<_> = lines.next().unwrap();
        lines.fold(init, |s1, s2| &s1 & &s2).len()
    }).sum();
    println!("Problem2 {}", res);
}
