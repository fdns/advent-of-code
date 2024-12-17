use std::ops::{BitAnd, BitXor};

use regex::Regex;

const INPUT: &str = include_str!("input/n17.input");

// 3-bit computer
// registers A B C, any integer
// 8 instructions: reads 3 bits after as input
// PC: next opcode, starts at 0 pointing at 3-bit number. Increses by 2 (opcode+operand), except for jump
// Past end of program, halt

// Operation:
const ADV: u8 = 0; // Division, num=A, den=2**(combo), truncated to int into A
const BLX: u8 = 1; // bitwise XOR of B/(literal), store in B
const BST: u8 = 2; // (combo) modulo 8 (keeping 3 bits), write into B
const JNZ: u8 = 3; // Nothing if A=0, PC=(literal); PC not increase
const BXC: u8 = 4; // B xor C, store in B, ignore (operand)
const OUT: u8 = 5; // (combo) mod8, outputs val (comma separated)
const BDV: u8 = 6; // Same as ADV, result stored in B
const CDV: u8 = 7; // Same as ADV, result stored in C

enum Registers {
    A = 0,
    B = 1,
    C = 2,
}

#[derive(Debug, Clone)]
struct Machine {
    pc: u32,
    registers: [u64; 3], // ABC
    program: Vec<u8>,
}

fn input() -> Machine {
    let mut machine = Machine {
        pc: 0,
        registers: [0, 0, 0],
        program: Vec::new(),
    };
    let registers = Regex::new(r"Register [ABC]: (?<val>\d+)").unwrap();

    let mut lines = INPUT.trim().lines();
    for i in 0..machine.registers.len() {
        let val = registers
            .captures(lines.next().unwrap())
            .unwrap()
            .name("val")
            .unwrap();
        machine.registers[i] = val.as_str().parse().unwrap();
    }

    let _ = lines.next().unwrap(); // separator

    let program = Regex::new(r"Program: (?<prog>[0-9,]+)").unwrap();
    for mem in program
        .captures(lines.next().unwrap())
        .unwrap()
        .name("prog")
        .unwrap()
        .as_str()
        .split(",")
    {
        machine.program.push(mem.parse().unwrap());
    }

    return machine;
}

fn execute(machine: &mut Machine) -> Vec<u8> {
    let mut output = Vec::new();

    loop {
        if !step(machine, &mut output) {
            break;
        }
    }

    return output;
}

// 17323786
//

fn step(machine: &mut Machine, output: &mut Vec<u8>) -> bool {
    if machine.program.len() - 1 <= machine.pc as usize {
        return false;
    }
    // println!(
    //     "Step {}, op {} {}, {:?}",
    //     machine.pc,
    //     machine.program[machine.pc as usize],
    //     machine.program[machine.pc as usize + 1],
    //     machine.registers,
    // );

    let opcode = machine.program[machine.pc as usize];
    let input = machine.program[machine.pc as usize + 1];
    match opcode {
        ADV => {
            // Division, num=A, den=2**(combo), truncated to int into A
            let a = machine.registers[Registers::A as usize];
            let b = combo(&machine, input);
            machine.registers[Registers::A as usize] = a / 2u64.pow(b.try_into().unwrap());
            machine.pc += 2;
        }
        BLX => {
            // bitwise XOR of B/(literal), store in B
            let a = machine.registers[Registers::B as usize];
            let b = input;
            machine.registers[Registers::B as usize] = a.bitxor(b as u64);
            machine.pc += 2;
        }
        BST => {
            // (combo) modulo 8 (keeping 3 bits), write into B
            let a = combo(&machine, input);
            let b = 7;
            machine.registers[Registers::B as usize] = a.bitand(b);
            machine.pc += 2;
        }
        JNZ => {
            // Nothing if A=0, PC=(literal); PC not increase
            if machine.registers[Registers::A as usize] != 0 {
                machine.pc = input as u32;
            } else {
                machine.pc += 2;
            }
        }
        BXC => {
            // B xor C, store in B, ignore (operand)
            let a = machine.registers[Registers::B as usize];
            let b = machine.registers[Registers::C as usize];
            machine.registers[Registers::B as usize] = a.bitxor(b);
            machine.pc += 2;
        }
        OUT => {
            // (combo) mod8, outputs val (comma separated)
            let a = combo(&machine, input);
            output.push(a.bitand(7) as u8);
            machine.pc += 2;
        }
        BDV => {
            // Same as ADV, result stored in B
            let a = machine.registers[Registers::A as usize];
            let b = combo(&machine, input);
            machine.registers[Registers::B as usize] = a / 2u64.pow(b.try_into().unwrap());
            machine.pc += 2;
        }
        CDV => {
            // Same as ADV, result stored in C
            let a = machine.registers[Registers::A as usize];
            let b = combo(&machine, input);
            machine.registers[Registers::C as usize] = a / 2u64.pow(b.try_into().unwrap());
            machine.pc += 2;
        }
        _ => unimplemented!("Invalid opcode {}", opcode),
    };

    return true;
}

fn combo(m: &Machine, input: u8) -> u64 {
    match input {
        0 | 1 | 2 | 3 => return input as u64,
        4 => m.registers[Registers::A as usize],
        5 => m.registers[Registers::B as usize],
        6 => m.registers[Registers::C as usize],
        _ => unimplemented!("Invalid combo value"),
    }
}

fn main() {
    p1();
    p2();
}
fn p1() {
    let mut m = input();
    let result = execute(&mut m);
    println!(
        "Result 1: {}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}
fn p2() {
    let m = input();
    let a = execute_match(&m);
    println!("Result 2: {}", a);
}

fn execute_match(og_machine: &Machine) -> u64 {
    // The program takes the first three bits to generate the output on each iteration
    // Let's patch the machine so it only does one iteration
    let mut machine = Machine {
        pc: 0,
        registers: [0; 3],
        program: og_machine.program.clone(),
    };
    machine.program.truncate(machine.program.len() - 2);

    let result = execute_nested(og_machine, &machine, og_machine.program.len() - 1, 0).unwrap();

    // Validate
    let mut validator = og_machine.clone();
    validator.registers[Registers::A as usize] = result;
    let validate = execute(&mut validator);
    println!("Validator: {}", format(validate));

    return result;
}

fn execute_nested(
    validator: &Machine,
    machine: &Machine,
    index: usize,
    current: u64,
) -> Option<u64> {
    for i in 0..8 {
        let next = (current << 3) + i;
        let mut m = machine.clone();
        m.registers[Registers::A as usize] = next;

        let val = execute(&mut m);
        if val[0] != validator.program[index] {
            continue;
        }

        // Found match, check if we are at the end
        if index == 0 {
            // Validate
            let mut validator = validator.clone();
            validator.registers[Registers::A as usize] = next;

            let result = execute(&mut validator);
            if result.eq(&validator.program) {
                return Some(next);
            }
            return None;
        }

        // Dig deeper
        if let Some(res) = execute_nested(validator, machine, index - 1, next) {
            return Some(res);
        }
    }
    return None;
}

fn format(r: Vec<u8>) -> String {
    return r
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
}
