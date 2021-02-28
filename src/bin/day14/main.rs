mod part2;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Mask {
    zero: usize,
    one: usize,
}

impl Mask {
    fn mask(&self, v:usize) -> usize {
        (v | self.one) & self.zero
    }

    fn new() -> Self {
        Self { zero: 0, one: 0 }
    }
}

impl From<&str> for Mask {
    fn from(s: &str) -> Self {
        let mut one:usize = 0 ;
        let mut zero:usize = 0;
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '1' => one += 1 << i,
                '0' => zero += 1 << i,
                _ => continue,
            }
        }
        Self{ zero: !zero, one}
    }
}

struct CPU {
    mask: Mask,
    memory: HashMap<usize, usize>
}

impl CPU {
    fn new() -> Self {
        Self{ mask: Mask::new(), memory: Default::default() }
    }

    fn exec_prg(&mut self, s:&str) {
        let re = Regex::new(r"mask = (.*)\n((?:mem.*\n?)*)").unwrap();
        let re_mem =Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        let batches = re.captures_iter(s);
        for b in batches {
            self.mask = Mask::from(&b[1]);
            let mem_ops =  re_mem.captures_iter(&b[2]);
            for m in mem_ops {
                let i = m[1].parse::<usize>().unwrap();
                let res = self.mask.mask(m[2].parse().unwrap());
                *self.memory.entry(i).or_insert(0) = res;
            }

        }

    }
}


#[test]
fn test_new_mask() {
    const PRG:&str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    let mut cpu = CPU::new();
    cpu.exec_prg(PRG);
    assert_eq!(64, cpu.memory[&8]);
    assert_eq!(165, cpu.memory.into_iter().map(|(_, v)| v).sum::<usize>());

}

fn main() {
    let mut cpu = CPU::new();
    let s = include_str!("input.txt");
    cpu.exec_prg(&s);
    println!("Problem 1: {}", cpu.memory.into_iter().map(|(_, v)| v).sum::<usize>());
    part2::main();
}
