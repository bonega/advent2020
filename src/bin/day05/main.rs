
fn string_to_dec(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, x| match x {
            'B' | 'R' => acc * 2 + 1,
            'F' | 'L' => acc * 2,
            _ => unimplemented!()
        })
}

fn string_to_seat(s: &str) -> usize {
    let row = string_to_dec(&s[..7]);
    let col = string_to_dec(&s[7..10]);
    row * 8 + col
}

#[test]
fn test_string_to_seat_id() {
    assert_eq!(567, string_to_seat("BFFFBBFRRR"));
    assert_eq!(119, string_to_seat("FFFBBBFRRR"));
    assert_eq!(820, string_to_seat("BBFFBBFRLL"));
}

fn main() {
    let s = include_str!("input.txt");
    let mut ids: Vec<_> = s.lines()
        .map(string_to_seat)
        .collect();
    ids.sort();
    println!("Problem1 {}", ids.iter().max().unwrap());

    let res = ids.windows(2).find(|x| x[1] - x[0] == 2).unwrap();
    println!("Problem2 {}", res[0] + 1);
}
