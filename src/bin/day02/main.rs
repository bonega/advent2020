use regex::Regex;

fn main() {
    let s = include_str!("input.txt");
    let count = s.lines().filter(|x| is_valid(x)).count();
    println!("Problem1: {}", count);
    let count = s.lines().filter(|x| is_valid2(x)).count();
    println!("Problem2: {}", count);
}

struct PasswordLine {
    first: usize,
    second: usize,
    token: char,
    text: String,
}


fn is_valid(s: &str) -> bool {
    let p = PasswordLine::new(s);
    let count = p.text.matches(p.token).count();
    (p.first..p.second + 1).contains(&count)
}

fn is_valid2(s: &str) -> bool {
    let p = PasswordLine::new(s);
    let first = p.text.chars().nth(p.first - 1).unwrap();
    let second = p.text.chars().nth(p.second - 1).unwrap();
    (first == p.token) ^ (second == p.token)
}

impl PasswordLine {
    fn new(s: &str) -> Self {
        let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<c>.): (?P<pass>.+)$").unwrap();
        let cap = re.captures(s).unwrap();
        PasswordLine {
            first: cap["min"].parse().unwrap(),
            second: cap["max"].parse().unwrap(),
            token: cap["c"].parse().unwrap(),
            text: cap["pass"].to_string(),
        }
    }
}

#[test]
fn test_is_valid() {
    assert_eq!(true, is_valid("1-3 a: abcde"));
    assert_eq!(false, is_valid("1-3 b: cdefg"));
    assert_eq!(true, is_valid("2-9 c: ccccccccc"));
}


#[test]
fn test_is_valid2() {
    assert_eq!(true, is_valid2("1-3 a: abcde"));
    assert_eq!(false, is_valid2("1-3 b: cdefg"));
    assert_eq!(false, is_valid2("2-9 c: ccccccccc"));
}

