use regex::{Regex, Captures};
use std::ops::Range;
use crate::ValidationError::ParseError;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RE: Regex = Regex::new(r"(?:byr:(\S+)|iyr:(\S+)|eyr:(\S+)|hgt:(\S+)|hcl:(\S+)|ecl:(\S+)|pid:(\S+)|cid:(\S+)|\s)+?(?:\n\n|\n$)").unwrap();
}


fn main() {
    let buffer = include_str!("input.txt");
    problem1(buffer);
    problem2(buffer);
}

fn problem1(buffer: &str) {
    let captures = RE.captures_iter(buffer);
    let x = captures.filter(|x| {
        x
            .iter()
            .skip(1)
            .take(7)
            .all(|y| y.is_some())
    }).count();
    println!("problem1: {}", x);
}

fn validate(captures: &Captures) -> Result<(), ValidationError> {
    use ValidationError::*;
    let passport_str = captures.get(0).unwrap().as_str();
    let passport_range = captures.get(0).unwrap().range();

    let m = captures.get(1).ok_or(ByrMissingError(passport_str.into(), passport_range.clone()))?;
    let s = m.as_str().to_owned();
    validate_byr(s.as_str())
        .map_err(|_| ByrParseError(s.into(), m.range()))?;

    let m = captures.get(2).ok_or(IyrMissingError(passport_str.into(), passport_range.clone()))?;
    let s = m.as_str().to_owned();
    validate_iyr(s.as_str())
        .map_err(|_| IyrParseError(s.into(), m.range()))?;

    let m = captures.get(3).ok_or(EyrMissingError(passport_str.into(), passport_range.clone()))?;
    let s = m.as_str().to_owned();
    validate_eyr(s.as_str())
        .map_err(|_| EyrParseError(s.into(), m.range()))?;

    let m = captures.get(4).ok_or(HgtMissingError(passport_str.into(), passport_range.clone()))?;
    let s = m.as_str().to_owned();
    validate_hgt(s.as_str())
        .map_err(|_| HgtParseError(s.into(), m.range()))?;

    let m = captures.get(5).ok_or(HclMissingError(passport_str.into(), passport_range.clone()))?;
    let s = m.as_str().to_owned();
    validate_hcl(m.as_str())
        .map_err(|_| HclParseError(s.into(), m.range()))?;

    let m = captures.get(6).ok_or(EclMissingError(passport_str.into(), passport_range.clone()))?;
    let s = m.as_str().to_owned();
    validate_ecl(m.as_str())
        .map_err(|_| EclParseError(s.into(), m.range()))?;

    let m = captures.get(7).ok_or(PidMissingError(passport_str.into(), passport_range.clone()))?;
    let s = m.as_str().to_owned();
    validate_pid(m.as_str())
        .map_err(|_| PidParseError(s.into(), m.range()))?;


    Ok(())
}

fn validate_pid(s: &str) -> ValidationResult {
    Regex::new(r"^\d{9}$").unwrap()
        .captures(s)
        .ok_or(ParseError).and(Ok(()))
}

fn validate_ecl(s: &str) -> ValidationResult {
    let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    re.captures(s).ok_or(ParseError).and(Ok(()))
}

fn validate_byr(s: &str) -> ValidationResult {
    match s.parse::<usize>() {
        Ok(1920..=2002) => Ok(()),
        _ => Err(ParseError),
    }
}


fn validate_iyr(s: &str) -> ValidationResult {
    match s.parse::<usize>() {
        Ok(2010..=2020) => Ok(()),
        _ => Err(ParseError),
    }
}

fn validate_hgt(s: &str) -> ValidationResult {
    let re = Regex::new(r"(\d+)(cm|in)").unwrap();
    let m = re.captures(s)
        .ok_or(ParseError)?;
    let hgt = m.get(1).ok_or(ParseError)?.as_str().parse::<usize>().map_err(|_| ParseError)?;
    let unit = m.get(2).ok_or(ParseError)?.as_str();
    match (hgt, unit) {
        (150..=193, "cm") => Ok(()),
        (59..=76, "in") => Ok(()),
        _ => Err(ParseError)
    }
}

type ValidationResult = Result<(), ValidationError>;

#[derive(Debug)]
enum ValidationError {
    HclParseError(String, Range<usize>),
    HclMissingError(String, Range<usize>),
    HgtParseError(String, Range<usize>),
    HgtMissingError(String, Range<usize>),
    EyrParseError(String, Range<usize>),
    EyrMissingError(String, Range<usize>),
    IyrParseError(String, Range<usize>),
    IyrMissingError(String, Range<usize>),
    ByrParseError(String, Range<usize>),
    ByrMissingError(String, Range<usize>),
    EclParseError(String, Range<usize>),
    EclMissingError(String, Range<usize>),
    PidParseError(String, Range<usize>),
    PidMissingError(String, Range<usize>),
    ParseError,
}

fn validate_eyr(s: &str) -> ValidationResult {
    match s.parse::<usize>() {
        Ok(2020..=2030) => Ok(()),
        _ => Err(ParseError),
    }
}

fn validate_hcl(s: &str) -> ValidationResult {
    let re = Regex::new(r"^#([a-z0-9]{6})$").unwrap();
    re.captures(s)
        .ok_or(ParseError)?
        .get(1)
        .ok_or(ParseError)
        .and(Ok(()))
}

#[test]
fn test_validate_ecl() {
    assert_eq!(true, validate_hcl("#123abc").is_ok());
    assert_eq!(false, validate_hcl("123abc").is_ok());
}

fn problem2(buffer: &str) {
    let captures = RE.captures_iter(buffer);
    let res = captures.filter_map(|x|validate(&x).ok());
    println!("Problem2: {}", res.count());
}
