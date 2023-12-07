use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    let binding = fs::read_to_string("input/day5.txt").unwrap();
    let mut lines = binding.lines();

    let seeds_line = lines.next().unwrap();
    let starting_seeds = seeds_line
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut mappings = Vec::new(); // ((from, to), Vec<(dest_start, src_start, length)>)

    let header_regex = regex::Regex::new(r"([a-z]+)-to-([a-z]+) map").unwrap();

    for line in lines {
        if let Some(cap) = header_regex.captures(line) {
            let from = cap.get(0).unwrap().as_str();
            let to = cap.get(1).unwrap().as_str();
            mappings.push(((from, to), Vec::new()));
        } else {
            if let Some((dest_start, src_start, length)) = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
            {
                mappings
                    .last_mut()
                    .unwrap()
                    .1
                    .push((dest_start, src_start, length));
            }
        }
    }

    let mut current = starting_seeds;
    for (_, this_mapping) in mappings.iter() {
        let previous_list = current.clone();
        current.clear();
        for previous in previous_list.iter() {
            let mut new = *previous;
            for (dest_start, src_start, length) in this_mapping.iter() {
                if previous >= src_start && previous < &(src_start + length) {
                    let offset = previous - src_start;
                    new = *dest_start + offset;
                    break;
                }
            }
            current.push(new);
        }
    }

    println!("{:?}", current.iter().min().unwrap());
}
