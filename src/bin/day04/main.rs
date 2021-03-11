use std::ops::RangeInclusive;

use anyhow::{anyhow, Context, Result, bail};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref PROBLEM1_RE: Regex = Regex::new(r"(?:byr:(\S+)|iyr:(\S+)|eyr:(\S+)|hgt:(\S+)|hcl:(\S+)|ecl:(\S+)|pid:(\S+)|cid:(\S+)|\s)+?(?:\n\n|\n$)").unwrap();
    pub static ref HCL_RE: Regex = Regex::new(r"^#([a-z0-9]{6})$").unwrap();
    pub static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    pub static ref ECL_RE: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
}


fn main() {
    let buffer = include_str!("input.txt");
    problem1(buffer);
    problem2(buffer);
}

fn problem1(buffer: &str) {
    let captures = PROBLEM1_RE.captures_iter(buffer);
    let x = captures.filter(|x| {
        x
            .iter()
            .skip(1)
            .take(7)
            .all(|y| y.is_some())
    }).count();
    println!("problem1: {}", x);
}

fn problem2(buffer: &str) {
    let blocks = buffer.split_terminator("\n\n");
    let valid = blocks.filter(|x| is_valid(x).is_ok());
    println!("Problem2: {}", valid.count());
}

fn is_valid(block: &str) -> Result<()> {
    let re = Regex::new(r"(?:byr:(\S+)|iyr:(\S+)|eyr:(\S+)|hgt:(\S+)|hcl:(\S+)|ecl:(\S+)|pid:(\S+)|cid:(\S+)|\s)+")?;
    let cap = re.captures(block).context("")?;
    str_in_range(cap.get(1).context("")?.as_str(), 1920..=2002)?;
    str_in_range(cap.get(2).context("")?.as_str(), 2010..=2020)?;
    str_in_range(cap.get(3).context("")?.as_str(), 2020..=2030)?;
    validate_hgt(cap.get(4).context("")?.as_str())?;
    validate_re(cap.get(5).context("")?.as_str(), &HCL_RE)?;
    validate_re(cap.get(6).context("")?.as_str(), &ECL_RE)?;
    validate_re(cap.get(7).context("")?.as_str(), &PID_RE)?;
    Ok(())
}

fn validate_re(s: &str, re: &Regex) -> Result<()> {
    match re.is_match(s) {
        true => Ok(()),
        false => Err(anyhow!("")),
    }
}

fn str_in_range(s: &str, range: RangeInclusive<usize>) -> Result<()> {
    let x = s.parse::<usize>()?;
    match range.contains(&x) {
        true => Ok(()),
        false => bail!(""),
    }
}

fn validate_hgt(s: &str) -> Result<()> {
    let re = Regex::new(r"(\d+)(cm|in)")?;
    let cap = re.captures(s).context("")?;
    let hgt = cap.get(1).context("")?.as_str().parse::<usize>()?;
    let unit = cap.get(2).context("")?.as_str();
    match (hgt, unit) {
        (150..=193, "cm") => Ok(()),
        (59..=76, "in") => Ok(()),
        _ => bail!("invalid range"),
    }
}
