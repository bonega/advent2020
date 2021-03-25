fn main() {
    let s = include_str!("input.txt");
    let numbers: Vec<_> = s.lines().map(|s| s.parse().unwrap()).collect();

    println!("Problem1 {}", problem1(&numbers));
    println!("Problem2 {}", problem2(&numbers));
}

fn problem1(numbers: &[usize]) -> usize {
    for (i, a) in numbers.iter().enumerate() {
        for b in &numbers[i + 1..] {
            if a + b == 2020 {
                return a * b;
            }
        }
    };
    unreachable!()
}


fn problem2(numbers: &[usize]) -> usize {
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers[i + 1..].iter().enumerate() {
            for c in &numbers[i + j + 1..] {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    };
    unreachable!();
}

