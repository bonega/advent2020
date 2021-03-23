use std::collections::HashMap;

fn main() {
    let p1_input = vec![0, 14, 1, 3, 7, 9];
    let mut memories = Memories::new(p1_input);
    println!("Problem1: {}", memories.nth(2019).unwrap());

    let p1_input = vec![0, 14, 1, 3, 7, 9];
    let mut memories = Memories::new(p1_input);
    println!("Problem2: {}", memories.nth(29999999).unwrap());
}

#[derive(Debug)]
struct Memories {
    last_spoken: Option<usize>,
    index: usize,
    numbers_map: HashMap<usize, usize>,
    seed: Vec<usize>,
}

impl Memories {
    fn new(mut starting_numbers: Vec<usize>) -> Self {
        starting_numbers.reverse();
        Self {
            last_spoken: None,
            index: 0,
            numbers_map: HashMap::new(),
            seed: starting_numbers,
        }
    }
}

impl Iterator for Memories {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_spoken = if let Some(x) = self.seed.pop() {
            if let Some(last_spoken) = self.last_spoken {
                self.numbers_map.insert(last_spoken, self.index);
            }
            Some(x)
        } else {
            let last_spoken = self.last_spoken.unwrap();
            let current = self.numbers_map.get(&last_spoken).map(|x| self.index - x).or(Some(0));
            self.numbers_map.insert(last_spoken, self.index);
            current
        };
        self.index += 1;
        self.last_spoken
    }
}

#[test]
fn test_memories() {
    let starting_numbers = vec![0, 3, 6];
    let memories = Memories::new(starting_numbers.clone());
    let res: Vec<_> = memories.skip(3).take(6).collect();
    assert_eq!(vec![0, 3, 3, 1, 0, 4], res);

    let mut memories = Memories::new(starting_numbers);
    assert_eq!(436, memories.nth(2019).unwrap());

    let starting_numbers = vec![1, 3, 2];
    let mut memories = Memories::new(starting_numbers);
    assert_eq!(1, memories.nth(2019).unwrap());
}