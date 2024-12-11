use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut stones = input
        .split(' ')
        .map(|s| (s.parse::<u64>().unwrap(), 1u64))
        .collect::<HashMap<u64, u64>>();

    for _ in 1..=75 {
        let mut new_stones = HashMap::<u64, u64>::new();
        for stone in stones.keys() {
            if *stone == 0 {
                // Take all zeroes, and move them to ones
                new_stones.insert(1, new_stones.get(&1).unwrap_or(&0) + stones[&0]);
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let new_rock = stone / 10u64.pow(digits / 2);
                    new_stones.insert(new_rock, new_stones.get(&new_rock).unwrap_or(&0) + stones[stone]);

                    let new_rock = stone % 10u64.pow(digits / 2);
                    new_stones.insert(new_rock, new_stones.get(&new_rock).unwrap_or(&0) + stones[stone]);

                } else {
                    let new_rock = stone * 2024;
                    new_stones.insert(new_rock, new_stones.get(&new_rock).unwrap_or(&0) + stones[stone]);
                }
            }
        }
        stones = new_stones;
    }

    println!("Result: {}", stones.into_values().sum::<u64>());
}
