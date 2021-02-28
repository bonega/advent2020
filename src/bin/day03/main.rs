const WIDTH: usize = 31;

#[derive(Copy, Clone, Debug)]
enum Square {
    Open,
    Tree,
}

struct Map {
    buffer: Vec<Square>
}

impl From<&[u8]> for Map {
    fn from(bytes: &[u8]) -> Self {
        let res: Vec<_> = bytes.iter()
            .filter(|x| **x != 10)
            .map(|x| match x {
            b'.' => Square::Open,
            b'#' => Square::Tree,
            _ => unreachable!(),
        }).collect();
        Map { buffer: res }
    }
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<Square> {
        let index = x % WIDTH + y * WIDTH;
        if index < self.buffer.len() {
            Some(self.buffer[index])
        } else { None }
    }
}

fn get_trees(map: &Map, right: usize, down: usize) -> usize {
    let mut x: usize = right;
    let mut y: usize = down;
    let mut trees: usize = 0;
    loop {
        match map.get(x, y) {
            Some(Square::Tree) => { trees += 1 }
            Some(Square::Open) => {}
            None => { break trees; }
        }
        x += right;
        y += down;
    }
}

fn main() {
    let buffer = include_bytes!("input.txt");
    let map = Map::from(&buffer[..]);
    let walk1 = get_trees(&map, 1, 1);
    let walk2 = get_trees(&map, 3, 1);
    let walk3 = get_trees(&map, 5, 1);
    let walk4 = get_trees(&map, 7, 1);
    let walk5 = get_trees(&map, 1, 2);
    println!("Problem1: {}", walk2);
    println!("Problem2: {}", walk1 * walk2 * walk3 * walk4 * walk5);
}


