use regex::Regex;
use anyhow::Result;
mod part2;

fn main() -> Result<()> {
    let s = include_str!("input.txt");
    let mut lines = s.lines();
    let start_time = lines.next().unwrap().parse::<usize>()?;
    let active_ids = str_to_ids(lines.next().unwrap());
    println!("Problem1: {}", problem1(start_time, active_ids));
    part2::main();
    Ok(())
}


#[test]
fn test_simple() {
    const SIMPLE_DATA: &str = "939
7,13,x,x,59,x,31,19";
    let lines: Vec<&str> = SIMPLE_DATA.lines().collect();
    let start_time = lines[0].parse::<usize>().unwrap();
    let active_ids = str_to_ids(lines[1]);
    let x = problem1(start_time, active_ids);
    assert_eq!(295, x);
}

fn str_to_ids(s: &str) -> Vec<usize> {
    let re = Regex::new(r"\d+").unwrap();
    let caps = re.captures_iter(s);
    caps.map(|x| x[0].parse().unwrap()).collect()
}


fn problem1(start_time: usize, active_ids: Vec<usize>) -> usize {
    (start_time..usize::max_value()).filter_map(|t| {
        active_ids.iter()
            .find(|i| t % *i == 0)
            .map(|i| i * (t - start_time))
    }).next().unwrap()
}

