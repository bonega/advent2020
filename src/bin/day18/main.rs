mod part1;
mod part2;


fn main() {
    let s = include_str!("input.txt");
    println!("Problem1: {}", part1::solve(s));
    println!("Problem2: {}", part2::solve(s));
}