mod part2;

use regex::Regex;
use std::ops::{RangeInclusive};
use anyhow::{Result, Context};

#[derive(Debug)]
struct Rule {
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl Rule {
    fn new(s: &str) -> Result<Self> {
        let re = Regex::new(r"(\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let m = re.captures(s).context("")?;
        let a1 = m[1].parse()?;
        let a2 = m[2].parse()?;
        let range1 = a1..=a2;
        let b1 = m[3].parse()?;
        let b2 = m[4].parse()?;
        let range2 = b1..=b2;
        Ok(Self { range1, range2 })
    }

    fn is_valid(&self, v: usize) -> bool {
        self.range1.contains(&v) || self.range2.contains(&v)
    }
}

fn parse_tickets(s: &str) -> usize {
    let re = Regex::new(r"(?s)(?P<rules>.*)\n(?P<my_ticket>your ticket:.*)nearby tickets:\n(?P<nearby_tickets>.*)").unwrap();
    let m = re.captures(s).unwrap();
    let rules:Vec<_> = m["rules"].lines().filter_map(|line| Rule::new(line).ok()).collect();
    let values: Vec<usize> = m["nearby_tickets"]
        .split(&[',', '\n'][..])
        .map(str::parse)
        .flatten().collect();
    let error_rate: usize = values.into_iter()
        .filter(|&x| !rules.iter()
            .any(|r| r.is_valid(x)))
        .sum();
    error_rate
}

#[test]
fn test_simple() {
    let s = include_str!("simple.txt");
    let error_rate = parse_tickets(&s);
    assert_eq!(71, error_rate);
}

fn main() {
    let s = include_str!("input.txt");
    let error_rate = parse_tickets(&s);
    println!("Problem 1: {}", error_rate);
    println!("Problem 2: {:?}", part2::solve(&s))
}
