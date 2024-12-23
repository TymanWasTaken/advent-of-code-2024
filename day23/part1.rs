use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // Assemble graph
    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for line in INPUT.lines() {
        let (first, second) = line.split_once('-').unwrap();
        graph.entry(first).or_default().push(second);
        graph.entry(second).or_default().push(first);
    }

    let mut combos = HashSet::<(&str, &str, &str)>::new();
    'outer: for combo in graph.keys().combinations(3) {
        if !combo.iter().any(|c| c.chars().next().unwrap() == 't') { continue }
        for (i, computer) in combo.iter().enumerate() {
            if i != 0 && !graph[*computer].contains(combo[0]) { continue 'outer }
            if i != 1 && !graph[*computer].contains(combo[1]) { continue 'outer }
            if i != 2 && !graph[*computer].contains(combo[2]) { continue 'outer }
        }
        combos.insert(combo.iter().map(|&&e| e).collect_tuple().unwrap());
    }
    dbg!(combos.len());
}
