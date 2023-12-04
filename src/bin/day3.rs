use aoc2023::grid::*;
use std::fs;

fn main() {
    let grid = Grid::new_with_lines(fs::read_to_string("input/day3.txt").unwrap().lines());
    let mut regions = Vec::new();

    for cell in grid.cells() {
        if cell.contents().unwrap().is_ascii_digit()
            && !regions
                .iter()
                .any(|(r, _): &(Region, u32)| r.contains(cell.location()))
        {
            let mut region = Region::new();

            let mut total = 0;
            for cell in cell
                .walk_inclusive(1, 0)
                .take_while(|c| c.contents().unwrap().is_ascii_digit())
            {
                total = total * 10 + cell.contents().unwrap().to_digit(10).unwrap();
                region.insert(cell.location());
            }

            regions.push((region, total));
        }
    }

    fn is_symbol(c: Option<char>) -> bool {
        c.map(|c| !c.is_ascii_digit() && c != '.').unwrap_or(false)
    }

    let total: u32 = regions
        .iter()
        .filter_map(|(region, total)| {
            if region.neighbors().any(|location| {
                let c = grid.cell(location).contents().copied();
                is_symbol(c)
            }) {
                Some(total)
            } else {
                None
            }
        })
        .sum();

    println!("{}", total);
}
