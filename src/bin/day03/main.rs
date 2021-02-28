const WIDTH: usize = 31;

fn get_trees(map: &[u8], right: usize, down: usize) -> usize {
    let mut x = right;
    let mut y = down;
    let mut trees = 0;
    loop {
        let index = x % WIDTH + y * (WIDTH + 1);
        match map.get(index) {
            Some(b'#') => { trees += 1 }
            Some(b'.') => {}
            Some(c) => unreachable!("{}", c),
            None => { break trees; }
        }
        x += right;
        y += down;
    }
}

fn main() {
    let buffer = include_bytes!("input.txt");
    let walk1 = get_trees(&buffer[..], 1, 1);
    let walk2 = get_trees(&buffer[..], 3, 1);
    let walk3 = get_trees(&buffer[..], 5, 1);
    let walk4 = get_trees(&buffer[..], 7, 1);
    let walk5 = get_trees(&buffer[..], 1, 2);
    println!("Problem1: {}", walk2);
    println!("Problem2: {}", walk1 * walk2 * walk3 * walk4 * walk5);
}


