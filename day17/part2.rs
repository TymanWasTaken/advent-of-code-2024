#![feature(vec_push_within_capacity)]
use std::collections::VecDeque;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

// I manually transpiled my program input to rust code, and then removed everything after the first print
// I hope this doesn't count as including my input, please have mercy o lord Eric Wastl
fn calculate(value: u64) -> u64 {
    let mut register = (value, 0, 0);

    register.1 = register.0 & 0b111; // Take the lowest 3 bits
    register.1 ^= 0b010; // XOR Depends on lowest 3 bits
    register.2 = register.0 / 2u64.pow(register.1.try_into().unwrap()); // Remove some bits of A depending on its last 3
    register.1 ^= 0b011; // XOR Depends on lowest 3 bits
    register.1 ^= register.2; // XOR Depends on lowest 3 and full
    return register.1 & 0b111;
}

fn main() {
    // Input parsing
    let (_, instructions) = INPUT.split_once("\n\n").unwrap();
    let instructions = instructions[9..]
        .trim_ascii_end()
        .split(",")
        .into_iter()
        .map(|num| {
            num.parse::<u64>().unwrap()
        })
        .collect_vec();
        
    // Since the program operates in chunks of 3
    // And since each chunk relies on the chunks above it
    // If we start from the last digit then we can get that right, and work from there

    // We have to use a queue/stack, because multiple inputs can get a correct digit, so we have to support branching
    let mut branches = VecDeque::<(u64, usize)>::with_capacity(instructions.len() * 3);
    branches.push_back((0b000, 0));
    let mut results = Vec::<u64>::new();
    loop {
        let Some((next, digits_calculated)) = branches.pop_back() else {
            break;
        };

        for i in 0b000..=0b111u64 {
            if calculate(next | i) == instructions[15 - digits_calculated] {
                if digits_calculated == 15 {
                    results.push(next | i);
                } else {
                    branches.push_back(((next | i) << 3, digits_calculated + 1));
                }
            }
        }
    }

    println!("Result: {}", results.iter().min().unwrap());
}
