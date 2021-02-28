use regex::Regex;
use std::collections::{BTreeSet};


#[derive(Debug)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        use Instruction::*;
        let argument: isize = s[4..].parse::<isize>().unwrap();
        match &s[0..3] {
            "acc" => ACC(argument),
            "jmp" => JMP(argument),
            "nop" => NOP(argument),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct CPU {
    acc: isize,
    pc: usize,
    instructions: Vec<Instruction>,
    visited_instructions: BTreeSet<usize>,
}

enum CPUError {
    NonTerminating,
}

enum State {
    Running,
    Finished,
}

impl From<&str> for CPU {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"([a-z]{3} .+)").unwrap();
        let instructions: Vec<Instruction> = re.captures_iter(s)
            .map(|caps| Instruction::from(caps.get(1).unwrap().as_str())).collect();
        CPU { acc: 0, pc: 0, instructions, visited_instructions: BTreeSet::new() }
    }
}

type ExecResult = Result<State, CPUError>;

impl CPU {
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
        if self.pc >= self.instructions.len() {
            Ok(State::Finished)
        } else {
            Ok(State::Running)
        }
    }

    fn run(&mut self) -> ExecResult {
        loop {
            match self.execute()? {
                State::Running => {}
                State::Finished => { return Ok(State::Finished); }
            }
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
        if let JMP(arg) = instructions[index] {
            instructions[index] = NOP(arg);
        } else if let NOP(arg) = instructions[index] {
            instructions[index] = JMP(arg);
        }
    }

    for i in 0..cpu.instructions.len() {
        cpu.reset();
        toggle_instruction(&mut cpu.instructions, i);
        match cpu.run() {
            Ok(State::Finished) => break,
            _ => {}
        }
        toggle_instruction(&mut cpu.instructions, i);
    }
}
fn main() -> anyhow::Result<()> {
    let s = include_str!("input.txt");
    let mut cpu: CPU = CPU::from(s);
    cpu.run();
    println!("Problem1: {}", cpu.acc);
    cpu.reset();
    monkey_patch(&mut cpu);
    println!("Problem2: {}", cpu.acc);
    Ok(())
}
