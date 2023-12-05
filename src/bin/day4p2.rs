use std::{collections::VecDeque, fs};

use itertools::Itertools;

fn main() {
    let num_regex = regex::Regex::new(r"\d+").unwrap();
    let cards: Vec<usize> = fs::read_to_string("input/day4.txt")
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

            holding.iter().filter(|num| winners.contains(num)).count()
        })
        .collect();

    let mut total = 0;
    let mut queue = VecDeque::new();
    for i in 0..cards.len() {
        queue.push_back(i);
    }

    while let Some(i) = queue.pop_front() {
        total += 1;
        let winners = cards[i];
        for count in 0..winners {
            let next = i + count + 1;
            queue.push_back(next);
        }
    }

    println!("{}", total);
}
