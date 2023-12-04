use aoc2023::grid::*;
use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct NumberedRegion {
    region: Region,
    total: u32,
}

fn main() {
    let grid = Grid::new_with_lines(fs::read_to_string("input/day3.txt").unwrap().lines());
    let mut regions = Vec::new();

    for cell in grid.cells() {
        if cell.contents().unwrap().is_ascii_digit()
            && !regions
                .iter()
                .any(|r: &NumberedRegion| r.region.contains(cell.location()))
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

            regions.push(NumberedRegion { region, total });
        }
    }

    fn is_symbol(c: Option<char>) -> bool {
        c.map(|c| !c.is_ascii_digit() && c != '.').unwrap_or(false)
    }

    let parts: Vec<_> = regions
        .into_iter()
        .filter(|r| {
            r.region.neighbors().any(|location| {
                let c = grid.cell(location).contents().copied();
                is_symbol(c)
            })
        })
        .collect();

    fn adjacent_parts<'a, T>(
        cell: &Cell<T>,
        parts: &'a [NumberedRegion],
    ) -> Vec<&'a NumberedRegion> {
        // We want unique by region but those don't have equality (and hash/ord are hard), so we use indexes.
        let indexes: HashSet<_> = cell
            .neighbors()
            .filter_map(|location| {
                parts.iter().enumerate().find_map(|(index, r)| {
                    if r.region.contains(location.location()) {
                        Some(index)
                    } else {
                        None
                    }
                })
            })
            .collect();

        indexes.into_iter().map(|index| &parts[index]).collect()
    }

    let total: u32 = grid
        .cells()
        .filter_map(|cell| {
            if cell.contents().map(|c| *c == '*').unwrap_or(false) {
                let adjacent_parts = adjacent_parts(&cell, &parts);

                if adjacent_parts.len() == 2 {
                    return Some(adjacent_parts[0].total * adjacent_parts[1].total);
                }
            }
            None
        })
        .sum();

    println!("{}", total);
}
