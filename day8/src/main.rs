use std::fs;
use std::collections::HashSet;

#[derive(Copy,Clone,PartialEq)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64)
}

impl Instruction {
    fn parse(inst: &str) -> Instruction {
        if inst.starts_with("acc") {
            let num = inst[5..].parse::<i64>().expect("Parse failed");
            if &inst[4..5] == "+" {
                return Instruction::Acc(num);
            } else {
                return Instruction::Acc(-num);
            }
        } else if inst.starts_with("jmp") {
            let num = inst[5..].parse::<i64>().expect("Parse failed");
            if &inst[4..5] == "+" {
                return Instruction::Jmp(num);
            } else {
                return Instruction::Jmp(-num);
            }
        } else if inst.starts_with("nop") {
            let num = inst[5..].parse::<i64>().expect("Parse failed");
            if &inst[4..5] == "+" {
                return Instruction::Nop(num);
            } else {
                return Instruction::Nop(-num);
            }
        } else {
            panic!("Bad instruction");
        }
    }

    // fn is_nop(&self) -> bool {match self {Instruction::Nop(_)=>true,_=>false}}
    fn is_acc(&self) -> bool {match self {Instruction::Acc(_)=>true,_=>false}}
    fn is_jmp(&self) -> bool {match self {Instruction::Jmp(_)=>true,_=>false}}
    fn get_val(&self) -> i64 {
        match self {
            Instruction::Nop(n) => *n,
            Instruction::Acc(n) => *n,
            Instruction::Jmp(n) => *n
        }
    }
}

// Halts when a repeat instruction is run, returns the repeated instruction and the acc value
fn execute_program(program: &Vec<Instruction>) -> (usize,i64) {
    let mut pc = 0usize;
    let mut acc = 0i64;
    let mut insts_run: HashSet<usize> = HashSet::new();
    while !insts_run.contains(&pc) && pc < program.len() {
        insts_run.insert(pc);
        match program[pc] {
            Instruction::Nop(_) => {pc += 1;},
            Instruction::Acc(n) => {acc += n; pc += 1;},
            Instruction::Jmp(n) => {pc = ((pc as i64) + n) as usize;}
        }
    }
    return (pc,acc);
}

// Uncorrupts the program; returns the index of the corrupted instruction and the final acc value.
fn uncorrupt(program: &Vec<Instruction>) -> (usize,i64) {
    for inst_to_fix in 0..program.len() {
        if program[inst_to_fix].is_acc() {continue;}
        let mut fixed_program = program.clone();
        if program[inst_to_fix].is_jmp() {
            fixed_program[inst_to_fix] = Instruction::Nop(program[inst_to_fix].get_val());
        } else {
            fixed_program[inst_to_fix] = Instruction::Jmp(program[inst_to_fix].get_val());
        }
        let (final_pc, final_acc) = execute_program(&fixed_program);
        if final_pc == program.len() {
            return (inst_to_fix, final_acc);
        }
    }
    return (0,0);
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut program: Vec<Instruction> = Vec::new();
    for line in file.lines() {
        program.push(Instruction::parse(line))
    }

    let (final_pc, final_acc) = execute_program(&program);
    println!("The first instruction to be repeated is at index {}.", final_pc);
    println!("The acc value before the first instruction repeats is {}.", final_acc);

    let (fixed_inst, correct_acc) = uncorrupt(&program);
    println!("The corrupted instruction was at index {}.", fixed_inst);
    println!("The correct acc value is {}.", correct_acc);
}
