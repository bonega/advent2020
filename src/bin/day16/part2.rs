use std::collections::HashSet;
use std::ops::RangeInclusive;

use regex::Regex;

#[derive(Debug)]
struct Rule {
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
    name: String,
}

impl Rule {
    fn is_valid(&self, v: usize) -> bool {
        self.range1.contains(&v) || self.range2.contains(&v)
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let m = re.captures(s).unwrap();
        let a1 = m[2].parse().unwrap();
        let a2 = m[3].parse().unwrap();
        let range1 = a1..=a2;
        let b1 = m[4].parse().unwrap();
        let b2 = m[5].parse().unwrap();
        let range2 = b1..=b2;
        Self { range1, range2, name: m[1].to_string() }
    }
}


fn parse_tickets(s: &str) -> usize {
    let re = Regex::new(r"(?s)(?P<rules>.*)\n(?P<my_ticket>your ticket:.*)nearby tickets:\n(?P<nearby_tickets>.*)").unwrap();
    let m = re.captures(s).unwrap();
    let rules: Vec<Rule> = m["rules"].lines().map(Rule::from).collect();
    let mut tickets: Vec<Vec<usize>> = m["nearby_tickets"]
        .lines()
        .map(|line| {
            line.split_terminator(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>()
        })
        .filter(|t| {
            t.iter()
                .all(|&v| rules.iter()
                    .any(|r| r.is_valid(v)))
        })
        .collect();
    let my_ticket: Vec<usize> = m["my_ticket"].split(&[',', '\n'][..]).map(str::parse).flatten().collect();
    tickets.push(my_ticket.clone());
    let nr_fields = rules.len();

    let mut rule_to_field: Vec<(usize, HashSet<_>)> = rules.iter()
        .enumerate()
        .map(|(i, r)| {
            let v: HashSet<usize> = (0..nr_fields).filter(|&j| {
                tickets.iter()
                    .all(|t| {
                        r.is_valid(t[j])
                    })
            }).collect();
            (i, v)
        })
        .collect();
    rule_to_field.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));
    let mut res: Vec<(usize, usize)> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for (i, v) in rule_to_field.iter() {
        res.push((*i, *v.difference(&seen).last().unwrap()));
        seen.extend(v);
    }
    res.sort_by(|(a, _), (b, _)| a.cmp(b));
    res.iter().take(6).map(|(_, i)| my_ticket[*i]).product()
}

pub(crate) fn solve(s: &str) -> usize {
    parse_tickets(s)
}

#[test]
fn test_solve() {
    let s = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    assert_eq!(1716, parse_tickets(s));
}
