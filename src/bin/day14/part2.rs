use std::collections::HashMap;

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
struct Mask {
    bitmask: usize,
    variation_bitmask: usize,
    variations: Vec<usize>,
}

impl Mask {
    fn addresses(&self, addr: usize) -> Vec<usize> {
        self.variations.iter().map(|x| x + (self.bitmask | (addr & !self.variation_bitmask))).collect()
    }

    fn new() -> Self {
        Self { bitmask: 0, variation_bitmask: 0, variations: vec![] }
    }
}

impl From<&str> for Mask {
    fn from(s: &str) -> Self {
        let mut bitmask = 0;
        let mut variation_bitmask = 0;
        let mut variations = Vec::new();
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '1' => bitmask += 1 << i,
                '0' => bitmask += 0 << i,
                'X' => {
                    let x = 1 << i;
                    variation_bitmask += x;
                    variations.push(x);
                }
                _ => unreachable!(),
            }
        }
        let mut res = Vec::new();
        comp_var(&variations, &mut res, 0);
        Self { bitmask, variation_bitmask, variations: res }
    }
}

fn comp_var(variants: &[usize], res: &mut Vec<usize>, acc: usize) {
    match variants {
        [] => res.push(acc),
        [x, tail @ ..] => {
            comp_var(tail, res, acc);
            comp_var(tail, res, acc + *x);
        }
    }
}

struct CPU {
    mask: Mask,
    memory: HashMap<usize, usize>,
}

impl CPU {
    fn new() -> Self {
        Self { mask: Mask::new(), memory: Default::default() }
    }

    fn exec_prg(&mut self, s: &str) -> Result<()> {
        let re = Regex::new(r"mask = (?P<mask>.*)\n(?P<memops>(?:mem.*\n?)*)")?;
        let re_mem = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)")?;
        let batches = re.captures_iter(s);
        for batch in batches {
            self.mask = Mask::from(&batch["mask"]);
            let mem_ops = re_mem.captures_iter(&batch["memops"]);
            for m in mem_ops {
                let base_addr = m["addr"].parse()?;
                let v = m["val"].parse()?;
                let addresses = self.mask.addresses(base_addr);
                for i in addresses.into_iter() {
                    self.memory.insert(i, v);
                }
            }
        }
        Ok(())
    }
}


#[test]
fn test_mask() {
    const PRG: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    let mut cpu = CPU::new();
    let _ = cpu.exec_prg(PRG);
    let sum = cpu.memory.into_iter().map(|(_, v)| v).sum::<usize>();
    assert_eq!(208, sum);
}

pub(crate) fn main() {
    let mut cpu = CPU::new();
    let s = include_str!("input.txt");
    let _ = cpu.exec_prg(&s);
    println!("Problem 2: {}", cpu.memory.into_iter().map(|(_, v)| v).sum::<usize>());
}
