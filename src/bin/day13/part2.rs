pub fn main() {
    let s = include_str!("input.txt");
    let mut lines = s.lines();
    lines.next();
    let active_ids = str_to_ids(lines.next().unwrap());
    println!("Problem2: {}", problem2(&active_ids));
}


fn problem2(active_ids: &[(usize, usize)]) -> usize {
    let acc = (active_ids[0].1, active_ids[0].1);
    active_ids.iter()
        .skip(1)
        .fold(acc, |(start, step_size), (dt, x)| {
            let mut res = (start..usize::MAX)
                .step_by(step_size)
                .filter(|t| (t + dt) % x == 0)
                .take(2);
            let res_offset = res.next().unwrap();
            let res_step_size = res.next().unwrap() - res_offset;
            (res_offset, res_step_size)
        }).0
}

#[test]
fn test_simple() {
    const SIMPLE_DATA: &str = "939
7,13,x,x,59,x,31,19";
    let mut lines = SIMPLE_DATA.lines();
    lines.next();
    let active_ids = str_to_ids(lines.next().unwrap());
    let x = problem2(&active_ids);
    assert_eq!(1068781, x);
}

fn str_to_ids(s: &str) -> Vec<(usize, usize)> {
    let caps = s.split(",");
    let mut res = Vec::new();
    for (i, c) in caps.enumerate() {
        if let Ok(c) = c.parse() {
            res.push((i, c));
        }
    }
    res
}
