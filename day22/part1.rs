#![feature(const_str_split_at)]
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
const INPUT: [u128; INPUT_LINES] = {
    let data = include_str!("input.txt");
    let mut nums = [0; INPUT_LINES];

    let mut i = 0;
    let mut count = 0;
    let mut last_newline = 0;
    while i < data.len() {
        if data.as_bytes()[i] == b'\n' {
            let Ok(parsed) = u128::from_str_radix(data.split_at(last_newline).1.split_at(i - last_newline).0.trim_ascii(), 10) else {
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
    let mut sum = 0u128;
    for mut secret in INPUT {
        for _ in 0..2000 {
            secret ^= secret * 64;
            secret %= 16777216;
            secret ^= secret / 32;
            secret %= 16777216;
            secret ^= secret * 2048;
            secret %= 16777216;
        }

        sum += secret;
    }

    println!("Result: {sum}");
}
