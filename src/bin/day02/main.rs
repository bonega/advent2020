use regex::Regex;


fn main() {
    let s = include_str!("input.txt");
    let policies:Vec<_> = s.lines().map(build_policy).collect();
    let count: usize = policies.iter().filter(|x| is_valid(x)).count();
    println!("Problem1: {}", count);
    let count: usize = policies.iter().filter(|x| is_valid2(x)).count();
    println!("Problem2: {}", count);
}

struct Policy {
    first: usize,
    second: usize,
    token: char,
    text: String,
}

fn is_valid(p: &Policy) -> bool {
    let count = p.text.matches(p.token).count();
    (p.first..p.second + 1).contains(&count)
}

fn is_valid2(p: &Policy) -> bool {
    (p.text.chars().nth(p.first - 1).unwrap() == p.token) ^ (p.text.chars().nth(p.second - 1).unwrap() == p.token)
}

fn build_policy(s: &str) -> Policy {
    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<c>.): (?P<pass>.+)$").unwrap();
    let cap = re.captures(s).unwrap();
    Policy {
        first: cap["min"].parse().unwrap(),
        second: cap["max"].parse().unwrap(),
        token: cap["c"].parse().unwrap(),
        text: cap["pass"].to_string(),
    }
}

#[test]
fn test_is_valid() {
    assert_eq!(true, is_valid(&build_policy("1-3 a: abcde")));
    assert_eq!(false, is_valid(&build_policy("1-3 b: cdefg")));
    assert_eq!(true, is_valid(&build_policy("2-9 c: ccccccccc")));
}


#[test]
fn test_is_valid2() {
    assert_eq!(true, is_valid2(&build_policy("1-3 a: abcde")));
    assert_eq!(false, is_valid2(&build_policy("1-3 b: cdefg")));
    assert_eq!(false, is_valid2(&build_policy("2-9 c: ccccccccc")));
}

