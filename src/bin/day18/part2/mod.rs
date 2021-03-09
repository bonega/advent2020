mod parser;

pub fn solve(s:&str) -> usize {
    s.lines().map(|line|parser::parse(line).unwrap()).sum()
}

#[test]
fn test_examples() {
    let s = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(231, parser::parse(s).unwrap());

    let s = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(51, parser::parse(s).unwrap());
}
