use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl Instruction {
    fn new(s: &str) -> Self {
        use Instruction::*;
        let argument = s[4..].parse().unwrap();
        match &s[0..3] {
            "acc" => ACC(argument),
            "jmp" => JMP(argument),
            "nop" => NOP(argument),
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct CPU {
    acc: isize,
    pc: usize,
    instructions: Vec<Instruction>,
    visited_instructions: HashSet<usize>,
}

enum CPUError {
    NonTerminating,
    OutOfBounds,
}

type ExecResult = Result<(), CPUError>;

impl CPU {
    fn new(s: &str) -> Self {
        let re = Regex::new(r"([a-z]{3} .+)").unwrap();
        let instructions:Vec<_> = re.captures_iter(s)
            .map(|caps| Instruction::new(caps.get(1).unwrap().as_str())).collect();
        CPU { acc: 0, pc: 0, instructions, visited_instructions: HashSet::new() }
    }
    fn jmp_rel(&mut self, x: isize) {
        self.visited_instructions.insert(self.pc);
        self.pc = self.pc.wrapping_add(x as usize);
    }

    fn execute(&mut self) -> ExecResult {
        use Instruction::*;
        if self.visited_instructions.contains(&self.pc) {
            return Err(CPUError::NonTerminating);
        }
        match self.instructions[self.pc] {
            ACC(arg) => {
                self.acc += arg;
                self.jmp_rel(1);
            }
            JMP(arg) => {
                self.jmp_rel(arg);
            }
            NOP(_) => self.jmp_rel(1),
        }
        match self.pc >= self.instructions.len() {
            true => Err(CPUError::OutOfBounds),
            false => Ok(()),
        }
    }

    fn run(&mut self) -> ExecResult {
        loop {
            self.execute()?
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.visited_instructions.clear();
    }
}

fn monkey_patch(cpu: &mut CPU) {
    fn toggle_instruction(instructions: &mut [Instruction], index: usize) {
        use Instruction::*;
        match instructions[index] {
            JMP(arg) => { instructions[index] = NOP(arg); }
            NOP(arg) => { instructions[index] = JMP(arg); }
            _ => {}
        }
    }

    for i in 0..cpu.instructions.len() {
        cpu.reset();
        toggle_instruction(&mut cpu.instructions, i);
        match cpu.run() {
            Err(CPUError::OutOfBounds) => break,
            _ => {}
        }
        toggle_instruction(&mut cpu.instructions, i);
    }
}

fn main() -> anyhow::Result<()> {
    let s = include_str!("input.txt");
    let mut cpu: CPU = CPU::new(s);
    let _ = cpu.run();
    println!("Problem1: {}", cpu.acc);
    cpu.reset();
    monkey_patch(&mut cpu);
    println!("Problem2: {}", cpu.acc);
    Ok(())
}
