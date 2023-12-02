use std::fs;

fn main() {
    let cube_regex = regex::Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut sum = 0;
    for line in fs::read_to_string("input/day2.txt").unwrap().lines() {
        let mut min_cubes = [0; 3];
        for take in line.split(";") {
            for cap in cube_regex.captures_iter(take) {
                let color = match &cap[2] {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };
                min_cubes[color] = min_cubes[color].max(cap[1].parse::<u32>().unwrap());
            }
        }

        sum += min_cubes[0] * min_cubes[1] * min_cubes[2];
    }

    println!("{sum}")
}
