use std::collections::HashMap;

use itertools::Itertools as _;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // Assemble graph
    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for line in INPUT.lines() {
        let (first, second) = line.split_once('-').unwrap();
        graph.entry(first).or_default().push(second);
        graph.entry(second).or_default().push(first);
    }

    let mut maximum_clique = Vec::<&&str>::new();
    for start_node in graph.keys() {
        let mut clique = Vec::<&&str>::from([start_node]);
        for node in graph.keys() {
            if clique.iter().all(|n| graph[node].contains(n)) {
                clique.push(node);
            }
        }

        if clique.len() > maximum_clique.len() {
            maximum_clique = clique;
        }
    }

    maximum_clique.sort();
    println!("Result: {}", maximum_clique.into_iter().join(","));
}
