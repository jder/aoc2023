use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day6.txt").unwrap();
    let (time, distance) = contents.lines().collect_tuple().unwrap();

    let num_regex = regex::Regex::new(r"(\d+)").unwrap();
    let times = num_regex
        .captures_iter(time)
        .map(|cap| cap.get(0).unwrap().as_str().parse().unwrap());
    let distances = num_regex
        .captures_iter(distance)
        .map(|cap| cap.get(0).unwrap().as_str().parse().unwrap());

    let result: usize = times
        .zip(distances)
        .map(|(time, distance)| {
            (0..=time)
                .filter(|held_time| (time - held_time) * held_time > distance)
                .count()
        })
        .product();

    println!("{result}");
}
