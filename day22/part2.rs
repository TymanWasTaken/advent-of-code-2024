#![feature(const_str_split_at)]

use std::collections::{HashMap, HashSet};
const INPUT_LINES: usize = {
    let bytes = include_bytes!("input.txt");
    let mut i = 0;
    let mut count = 0;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            count += 1;
        }
        i += 1;
    }

    count
};
const INPUT: [u64; INPUT_LINES] = {
    let data = include_str!("input.txt");
    let mut nums = [0; INPUT_LINES];

    let mut i = 0;
    let mut count = 0;
    let mut last_newline = 0;
    while i < data.len() {
        if data.as_bytes()[i] == b'\n' {
            let Ok(parsed) = u64::from_str_radix(data.split_at(last_newline).1.split_at(i - last_newline).0.trim_ascii(), 10) else {
                panic!();
            };
            nums[count] = parsed;
            count += 1;
            last_newline = i;
        }
        i += 1;
    }

    nums
};

fn main() {
    let mut sequences = HashMap::<[i8; 4], u64>::new();
    for mut secret in INPUT {
        let mut sequence: [i8; 4] = [0, 0, 0, 0];
        let mut used = HashSet::<[i8; 4]>::new();
        for i in 0..2000 {
            let last_bananas: i8 = (secret % 10).try_into().unwrap();
            secret ^= secret * 64;
            secret %= 16777216;
            secret ^= secret / 32;
            secret %= 16777216;
            secret ^= secret * 2048;
            secret %= 16777216;
            sequence[0] = sequence[1];
            sequence[1] = sequence[2];
            sequence[2] = sequence[3];
            sequence[3] = TryInto::<i8>::try_into(secret % 10).unwrap() - last_bananas;

            if i < 4 || used.contains(&sequence) { continue } // Skip first 4 numbers, they can never form a real sequence
            *sequences.entry(sequence).or_default() += secret % 10;
            used.insert(sequence);
        }
    }

    println!("Result: {}", sequences.values().max().unwrap());
}
