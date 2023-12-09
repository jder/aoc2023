use std::fs;

use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("input/day9.txt").unwrap();
    let sum: i64 = contents
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.parse::<i64>().unwrap())
                .collect_vec();
            predict_next(&values)
        })
        .sum();

    println!("{}", sum);
}

fn predict_next(values: &[i64]) -> i64 {
    let differences = values
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect_vec();
    if differences.iter().all(|d| *d == 0) {
        *values.last().unwrap()
    } else {
        values.last().unwrap() + predict_next(&differences)
    }
}
