mod part1;
mod part2;

fn main() {
    let s1 = include_str!("input.txt");
    let s2 = include_str!("part2/input_modified.txt");
    println!("Problem1: {}", part1::solve(s1));
    println!("Problem2: {}", part2::solve(s2));
}