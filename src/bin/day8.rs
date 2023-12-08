use std::{collections::HashMap, fs};

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

    let mut current_node = "AAA";
    let mut num_steps = 0;
    'done: loop {
        for dir in directions.chars() {
            match dir {
                'L' => current_node = edges.get(current_node).unwrap().0,
                'R' => current_node = edges.get(current_node).unwrap().1,
                _ => panic!("invalid direction"),
            }
            num_steps += 1;
            if current_node == "ZZZ" {
                break 'done;
            }
        }
    }

    println!("{num_steps}");
}
