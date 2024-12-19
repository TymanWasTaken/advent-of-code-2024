use std::collections::LinkedList;

use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

struct Register {
    a: u64,
    b: u64,
    c: u64,
}

enum Instruction {
    OpCode(OpCode),
    Operand(Operand)
}

#[repr(u8)]
#[derive(FromPrimitive, Clone, Copy)]
enum OpCode {
    /// Performs division
    /// A register <- (A register)/(2^(combo operand))
    /// !!round towards 0!!
    Adv = 0,
    /// Calculates bitwise XOR
    /// B register <- (B register) ^ (literal operand)
    Bxl = 1,
    /// Calculates modulo 8 (thereby keeping only its lowest 3 bits)
    /// (combo operand) % 8
    Bst = 2,
    /// Conditional jump
    /// If (A register) == 0: do nothing
    /// Else: Jump to literal operand
    /// !!if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.!!
    Jnz = 3,
    /// Calculates bitwise XOR
    /// B register <- (B register) ^ (C register)
    /// !!reads the operand, but ignores it!!
    Bxc = 4,
    /// Outputs data mod 8
    /// print ((combo operand) % 8)
    /// !!multiple outputs are separated by commas!!
    Out = 5,
    /// Performs division
    /// B register <- (A register) / (2^(combo operand))
    /// !!round towards 0!!
    Bdv = 6,
    /// Performs division
    /// C register <- (A register) / (2^(combo operand))
    /// !!round towards 0!!
    Cdv = 7,
}

#[repr(u8)]
enum Operand {
    Literal(u8),
    RegisterA = 4,
    RegisterB = 5,
    RegisterC = 6,
    Reserved  = 7
}

impl Operand {
    fn from_opcode(opcode: OpCode, num: u8) -> Self {
        match opcode {
            OpCode::Bxl | OpCode::Jnz | OpCode::Bxc => Operand::Literal(num),
            OpCode::Adv | OpCode::Bst | OpCode::Out | OpCode::Bdv | OpCode::Cdv => match num {
                0..=3 => Operand::Literal(num),
                4 => Operand::RegisterA,
                5 => Operand::RegisterB,
                6 => Operand::RegisterC,
                7 | _ => unreachable!(),
            },
        }
    }

    fn get_value(&self, register: &mut Register) -> u64 {
        match self {
            Operand::Literal(n) => (*n).into(),
            Operand::RegisterA => register.a,
            Operand::RegisterB => register.b,
            Operand::RegisterC => register.c,
            Operand::Reserved => unreachable!(),
        }
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    // Input parsing
    let (registers, instructions) = INPUT.split_once("\n\n").unwrap();
    let mut register = registers
        .split("\n")
        .map(|l| l[12..].parse::<u64>().unwrap())
        .enumerate()
        .fold(Register { a: 0, b: 0, c: 0 }, |mut acc, (i, v)| {
            match i {
                0 => acc.a = v,
                1 => acc.b = v,
                2 => acc.c = v,
                _ => unreachable!()
            };
            acc
        });
    let instructions = instructions[9..]
        .trim_ascii_end()
        .split(",")
        .chunks(2)
        .into_iter()
        .flat_map(|mut chunk| {
            let opcode = OpCode::from_u8(chunk.next().unwrap().parse::<u8>().unwrap()).unwrap();
            [
                Instruction::OpCode(opcode),
                Instruction::Operand(Operand::from_opcode(opcode, chunk.next().unwrap().parse::<u8>().unwrap()))
            ]
        })
        .collect_vec();
    let mut instruction_pointer = 0;

    // Actual logic
    loop {
        let Some(Instruction::OpCode(opcode)) = instructions.get(instruction_pointer) else {
            break;
        };
        let Some(Instruction::Operand(operand)) = instructions.get(instruction_pointer + 1) else {
            break;
        };
        let operand = operand.get_value(&mut register);
        
        match opcode {
            OpCode::Adv => register.a = register.a / 2u64.pow(operand.try_into().unwrap()),
            OpCode::Bxl => register.b = register.b ^ operand,
            OpCode::Bst => register.b = operand % 8,
            OpCode::Jnz => if register.a != 0 {
                instruction_pointer = operand.try_into().unwrap();
                continue;
            },
            OpCode::Bxc => register.b = register.b ^ register.c,
            OpCode::Out => print!("{},", register.b % 8),
            OpCode::Bdv => register.b = register.a / 2u64.pow(operand.try_into().unwrap()),
            OpCode::Cdv => register.c = register.a / 2u64.pow(operand.try_into().unwrap()),
        }
        
        instruction_pointer += 2;
    }

    // Erase last comma
    print!("\x08 \n");
}
