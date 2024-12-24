use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Gate<'a> {
    first: &'a str,
    second: &'a str,
    op: Operation
}

#[repr(u8)]
#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

fn solve<'a>(state: &mut HashMap<&'a str, u64>, gates: &'a HashMap<&'a str, Gate>, output: &'a str) -> u64 {
    state.get(output).map(|&t| t).unwrap_or_else(|| {
        let gate = &gates[output];
        let (first_res, second_res) = (solve(state, gates, gate.first), solve(state, gates, gate.second));
        state.entry(gate.first).or_insert(first_res);
        state.entry(gate.second).or_insert(second_res);
        match gate.op {
            Operation::And => first_res & second_res,
            Operation::Or => first_res | second_res,
            Operation::Xor => first_res ^ second_res
        }
    })
}

fn main() {
    let (state, gates) = INPUT.split("\n\n").collect_tuple().unwrap();
    let mut state = HashMap::<&str, u64>::from_iter(
        state
            .trim()
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .filter(|wire)
            .map(|(wire, value)| (wire, if value == "0" { 0 } else { 1 }))
    );
    let gates = HashMap::<&str, Gate>::from_iter(
        gates
            .trim()
            .lines()
            .map(|l| l.split_once(" -> ").unwrap())
            .map(|(inputs, output)| (inputs.split(' ').collect_tuple().unwrap(), output))
            .map(|((first, op, second), output)| (output, Gate { first, second, op: match op {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => unreachable!()
            }}))
    );

    let mut result = 0u64;

    for (&output, z_value) in gates.keys().filter_map(|k| k.split_once('z').map(|(_, num)| (k, num.parse::<u64>().unwrap()))) {
        result |= solve(&mut state, &gates, output) << z_value;
    }

    println!("{result}");
}
