use std::{collections::HashMap, fs};

use itertools::Itertools;
use num::Integer;

fn main() {
    let contents = fs::read_to_string("input/day8.txt").unwrap();
    let mut lines = contents.lines();

    let directions = lines.next().unwrap();

    let node_regex = regex::Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    let mut edges = HashMap::new();

    for line in lines {
        if let Some(captures) = node_regex.captures(line) {
            let node = captures.get(1).unwrap().as_str();
            let (left, right) = (
                captures.get(2).unwrap().as_str(),
                captures.get(3).unwrap().as_str(),
            );
            edges.insert(node, (left, right));
        }
    }

    let mut current_nodes: Vec<&str> = edges
        .keys()
        .filter(|node| node.chars().last().unwrap() == 'A')
        .copied()
        .collect_vec();

    let mut seen: HashMap<(usize, &str, usize), usize> = HashMap::new();
    let mut loops: HashMap<usize, (usize, usize, usize)> = HashMap::new();

    let mut num_steps = 0;
    'done: loop {
        for (index, dir) in directions.chars().enumerate() {
            match dir {
                'L' => {
                    current_nodes
                        .iter_mut()
                        .for_each(|node| *node = edges.get(node).unwrap().0);
                }
                'R' => {
                    current_nodes
                        .iter_mut()
                        .for_each(|node| *node = edges.get(node).unwrap().1);
                }
                _ => panic!("invalid direction"),
            }
            num_steps += 1;

            for (node_index, node) in current_nodes.iter().enumerate() {
                if loops.contains_key(&node_index) {
                    continue;
                }
                if node.chars().last().unwrap() != 'Z' {
                    continue;
                }
                if let Some(previous_steps) = seen.get(&(node_index, node, index)) {
                    loops.insert(node_index, (index, *previous_steps, num_steps));
                    println!("found loop for {node_index}: {node}, {index} at {num_steps}, previously {previous_steps} cycle length {}", num_steps - previous_steps);

                    if loops.len() == current_nodes.len() {
                        break 'done;
                    }
                } else {
                    seen.insert((node_index, node, index), num_steps);
                }
            }
        }
    }

    let loop_lengths = loops.values().map(|(_, previous, now)| now - previous);
    let lcm = loop_lengths.fold(1, |acc, length| acc.lcm(&length));

    println!("{}", lcm); // this only works because the cycle happens to be at the beginning of the sequence
}
