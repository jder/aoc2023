use std::fs;

fn main() {
    let id_regex = regex::Regex::new(r"Game (\d+):").unwrap();
    let cube_regex = regex::Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut sum = 0;
    for line in fs::read_to_string("input/day2.txt").unwrap().lines() {
        let id = id_regex.captures(line).unwrap()[1].parse::<u32>().unwrap();
        let mut ok = true;
        for take in line.split(";") {
            let mut cubes = [0; 3];

            for cap in cube_regex.captures_iter(take) {
                let color = match &cap[2] {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };
                cubes[color] += &cap[1].parse::<u32>().unwrap();
            }

            if cubes[0] > 12 || cubes[1] > 13 || cubes[2] > 14 {
                ok = false;
                break;
            }
        }
        if ok {
            sum += id;
        }
    }

    println!("{sum}")
}
