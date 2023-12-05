use std::fs;

use itertools::Itertools;

fn main() {
    let num_regex = regex::Regex::new(r"\d+").unwrap();
    let sum: u32 = fs::read_to_string("input/day4.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (winners, holding) = line
                .split_once(":")
                .unwrap()
                .1
                .split("|")
                .map(|nums| {
                    num_regex
                        .find_iter(nums)
                        .map(|num| num.as_str().parse().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect_tuple()
                .unwrap();

            let num_winners = holding.iter().filter(|num| winners.contains(num)).count();
            if num_winners == 0 {
                0
            } else {
                2_u32.pow(num_winners as u32 - 1)
            }
        })
        .sum();

    println!("{sum}");
}
