use itertools::Itertools;
use rayon::prelude::*;
use std::fs;

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
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple().unwrap())
        .collect::<Vec<(u64, u64)>>();

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

    let mut min = None;
    for (start, length) in starting_seeds.iter() {
        println!("{} {}", start, length);
        let my_min = (*start..(*start + *length))
            .into_par_iter()
            .map(|now| {
                let mut current = now;
                for (_, this_mapping) in mappings.iter() {
                    for (dest_start, src_start, length) in this_mapping.iter() {
                        if current >= *src_start && current < *src_start + length {
                            let offset = current - src_start;
                            current = *dest_start + offset;
                            break;
                        }
                    }
                }
                current
            })
            .min()
            .unwrap();
        if min.is_none() || min.unwrap() > my_min {
            min = Some(my_min);
        }
    }

    println!("{:?}", min);
}
