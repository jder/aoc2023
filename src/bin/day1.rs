use std::fs;

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input/day1.txt").unwrap().lines() {
        let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
        let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();

        let joined = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();
        sum += joined;
    }

    println!("{sum}")
}
