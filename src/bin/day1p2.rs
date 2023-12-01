use std::fs;

fn digit_value(location: &str) -> Option<u32> {
    if let Some(d) = location.chars().next().and_then(|c| c.to_digit(10)) {
        return Some(d);
    }

    for (value, string) in [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ] {
        if location.starts_with(string) {
            return Some(value);
        }
    }

    None
}

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input/day1.txt").unwrap().lines() {
        let substrings = line.char_indices().map(|(i, _)| &line[i..]);
        let first_digit = substrings.clone().find_map(digit_value).unwrap();
        let last_digit = substrings.clone().rev().find_map(digit_value).unwrap();

        let joined = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();
        sum += joined;
    }

    println!("{sum}")
}
