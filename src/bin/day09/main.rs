fn find_encoding_error(nums: &[usize], preamble_len: usize) -> usize {
    nums
        .windows(preamble_len + 1)
        .find_map(|chunk| {
            let (&checksum, preamble) = chunk.split_last()?;
            for (i, a) in preamble.iter().enumerate() {
                for b in &preamble[i + 1..] {
                    if a + b == checksum {
                        return None;
                    }
                }
            }
            Some(checksum)
        }).unwrap()
}

#[test]
fn test_find_encoding_error() {
    let nums = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
    assert_eq!(127, find_encoding_error(&nums, 5));
}

fn problem2(expected: usize, nums: &[usize]) -> Option<usize> {
    for size in 2..nums.len() {
        let res = nums
            .windows(size)
            .find(|chunk| expected == chunk.iter().sum());
        if let Some(res) = res {
            return Some(res.iter().min()? + res.iter().max()?);
        }
    };
    None
}

#[test]
fn test_find_problem2() {
    let nums = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
    assert_eq!(Some(62), problem2(127, &nums));
}

fn main() {
    let s = include_str!("input.txt");
    let nums: Vec<_> = s.lines().map(|x| x.parse().unwrap()).collect();
    let encoding_error = find_encoding_error(&nums, 25);
    println!("Problem1: {}", encoding_error);
    println!("Problem2: {}", problem2(encoding_error, &nums).unwrap());
}
