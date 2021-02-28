fn main() {
    let s = include_str!("input.txt");
    let numbers:Vec<u32> = s.lines().map(|s| s.parse().unwrap()).collect();

    println!("Problem1 {}", problem1(&numbers));
    println!("Problem2 {}", problem2(&numbers));
}

fn problem1(numbers: &[u32]) -> u32 {
    for (i, v) in numbers.iter().enumerate() {
        for v2 in &numbers[i..] {
            if v + v2 == 2020 {
                return v * v2;
            }
        }
    };
    unreachable!()
}


fn problem2(numbers: &[u32]) -> u32 {
    for (i, v) in numbers.iter().enumerate() {
        for (j, v2) in numbers[i..].iter().enumerate() {
            for v3 in &numbers[i + j..] {
                if v + v2 + v3 == 2020 {
                    return v * v2 * v3;
                }
            }
        }
    };
    unreachable!();
}

