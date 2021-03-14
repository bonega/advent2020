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

    fn build_helper(&self, index: usize) -> String {
        let rule = &self.rules[&index];
        let mut res = String::new();
        match rule {
            Rule::And(rules) => {
                for r in rules.iter() {
                    res.push_str(&self.build_helper(*r));
                }
            }
            Rule::Or(lhs, rhs) => {
                res.push_str("(");
                for r in lhs.iter() {
                    res.push_str(&self.build_helper(*r));
                }
                res.push_str("|");

                for r in rhs.iter() {
                    res.push_str(&self.build_helper(*r));
                }
                res.push_str(")");
            }
            Rule::Literal(lit) => {
                res.push_str(lit.to_string().as_ref());
            }
        }
        res
    }

    fn build_regex(&self, index: usize) -> String {
        format!("(?m)^{}$", self.build_helper(index))
    }
}


#[test]
fn test_consume() {
    let s = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
    assert_eq!(2, solve(s));
}

pub fn solve(s: &str) -> usize {
    let rules = s.split_terminator("\n\n").next().unwrap();
    let p = Parser::new(rules).unwrap();
    let re_str = p.build_regex(0);
    let re = Regex::new(re_str.as_str()).unwrap();
    re.captures_iter(s).count()
}
