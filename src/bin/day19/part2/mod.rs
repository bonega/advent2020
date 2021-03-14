use std::collections::HashMap;

use anyhow::Context;
use regex::Regex;

#[derive(Debug)]
enum Rule {
    And(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
    Literal(char),
}

#[derive(Debug)]
struct Parser {
    rules: HashMap<usize, Rule>
}

impl Parser {
    fn new(s: &str) -> anyhow::Result<Self> {
        let re = Regex::new(r"(\d+): ([^|\n]+)(?:\| (.+))?")?;
        let arm_re = Regex::new(r"\d+")?;
        let mut rules = HashMap::new();
        for i in s.lines() {
            let caps = re.captures(i).context("No match")?;
            let index = caps.get(1).context("Can't parse index")?.as_str().parse::<usize>()?;
            let left_arm = caps.get(2).context("Can't find left arm")?.as_str();
            if let Some(literal) = left_arm.chars().find(|&c| c == 'a' || c == 'b') {
                let literal = Rule::Literal(literal);
                rules.insert(index, literal);
                continue;
            }

            let left_arm: Vec<usize> = arm_re.captures_iter(left_arm).map(|m| m[0].parse().unwrap()).collect();

            if let Some(s) = caps.get(3).map(|m| m.as_str()) {
                let right_arm: Vec<usize> = arm_re.captures_iter(s).map(|m| m[0].parse().unwrap()).collect();
                rules.insert(index, Rule::Or(left_arm, right_arm));
            } else { rules.insert(index, Rule::And(left_arm)); }
        }
        Ok(Self { rules })
    }

    fn build_helper<'a>(&self, index: usize, input: &'a str) -> Vec<&'a str> {
        let rule = &self.rules[&index];
        return match rule {
            Rule::And(rules) => {
                self.parse_rules(input, rules)
            }
            Rule::Or(lhs, rhs) => {
                if index == 11 {
                    let mut output = self.parse_rules(input, &[42, 31]);
                    output.extend(self.parse_rules(input, &[42, 42, 31, 31]));
                    output.extend(self.parse_rules(input, &[42, 42, 42, 31, 31, 31]));
                    output.extend(self.parse_rules(input, &[42, 42, 42, 42, 31, 31, 31, 31]));
                    return output;
                }
                let mut output = self.parse_rules(input, lhs);
                output.extend(self.parse_rules(input, rhs));
                output
            }
            Rule::Literal(lit) => {
                if input.starts_with(*lit) {
                    vec![&input[1..]]
                } else {
                    vec![]
                }
            }
        };
    }

    fn parse_rules<'a>(&self, input: &'a str, rules: &[usize]) -> Vec<&'a str> {
        let mut input = vec![input];
        for r in rules.iter() {
            let mut output = Vec::new();
            for i in input.iter() {
                let res = self.build_helper(*r, i);
                output.extend(res);
            }
            input = output;
        }
        input.sort();
        input.dedup();
        input
    }

    fn is_match(&self, input: &str) -> bool {
        self.build_helper(0, input).iter().any(|x| *x == "")
    }
}


#[test]
fn test_consume() {
    let s = include_str!("test.txt");
    assert_eq!(3, solve(s));
}

#[test]
fn test_consume_modified() {
    let s = include_str!("test_modified.txt");
    assert_eq!(12, solve(s));
}

pub fn solve(s: &str) -> usize {
    let mut temp = s.split_terminator("\n\n");
    let rules = temp.next().unwrap();
    let input = temp.next().unwrap();
    let p = Parser::new(rules).unwrap();
    input.lines().filter(|x| p.is_match(x)).count()
}
