use std::collections::HashSet;

pub type Index = i32;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    contents: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn cell(&self, location: Location) -> Cell<T> {
        Cell {
            grid: self,
            location,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> {
        let width = self.width as Index;
        let height = self.height as Index;
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| Location { x, y }))
            .map(move |location| Cell {
                grid: self,
                location,
            })
    }

    pub fn map<U>(&self, mut f: impl FnMut(Cell<T>) -> U) -> Grid<U> {
        let mut contents = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut new_row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                new_row.push(f(Cell {
                    grid: self,
                    location: Location {
                        x: x as Index,
                        y: y as Index,
                    },
                }));
            }
            contents.push(new_row);
        }

        Grid {
            contents,
            width: self.width,
            height: self.height,
        }
    }
}

impl Grid<char> {
    pub fn new_with_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        let mut contents = Vec::new();
        let mut width = None;
        let mut height = 0;

        for line in lines {
            let mut row = Vec::new();
            for c in line.as_ref().chars() {
                row.push(c);
            }
            if let Some(w) = width {
                assert_eq!(w, row.len());
            } else {
                width = Some(row.len());
            }
            contents.push(row);
            height += 1;
        }

        Self {
            contents,
            width: width.unwrap_or_default(),
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub x: Index,
    pub y: Index,
}

impl Location {
    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Location> + '_ {
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .map(|(dx, dy)| self.offset(dx, dy))
    }
}

#[derive(Debug)]
pub struct Cell<'a, T> {
    grid: &'a Grid<T>,
    location: Location,
}

impl<'a, T> Cell<'a, T> {
    pub fn contents(&self) -> Option<&T> {
        if !self.in_bounds() {
            None
        } else {
            self.grid
                .contents
                .get(self.location.y as usize)
                .and_then(|row| row.get(self.location.x as usize))
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Cell<'a, T> {
        Cell {
            grid: self.grid,
            location: self.location.offset(dx, dy),
        }
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Cell<'a, T>> + '_ {
        self.location.neighbors().map(move |location| Cell {
            grid: self.grid,
            location,
        })
    }

    /// Walks in the given direction until it hits the edge of the grid.
    /// This cell is not included in the iterator.
    pub fn walk(&self, dx: i32, dy: i32) -> impl Iterator<Item = Cell<'a, T>> + '_ {
        let mut cell = *self;
        std::iter::from_fn(move || {
            cell = cell.offset(dx, dy);
            if cell.in_bounds() {
                Some(cell)
            } else {
                None
            }
        })
    }

    /// Walks in the given direction until it hits the edge of the grid.
    /// This cell is included in the iterator.
    pub fn walk_inclusive(&self, dx: i32, dy: i32) -> impl Iterator<Item = Cell<'a, T>> + '_ {
        std::iter::once(*self).chain(self.walk(dx, dy))
    }

    pub fn in_bounds(&self) -> bool {
        self.location.x >= 0
            && self.location.y >= 0
            && self.location.x < self.grid.width as Index
            && self.location.y < self.grid.height as Index
    }
}

impl<'a, T> Clone for Cell<'a, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            location: self.location,
        }
    }
}

impl<'a, T> Copy for Cell<'a, T> {}

#[derive(Debug, Clone)]
pub struct Region {
    locations: HashSet<Location>,
}

impl Region {
    pub fn new() -> Self {
        Self {
            locations: HashSet::new(),
        }
    }

    pub fn insert(&mut self, location: Location) {
        self.locations.insert(location);
    }

    pub fn contains(&self, location: Location) -> bool {
        self.locations.contains(&location)
    }

    pub fn iter(&self) -> impl Iterator<Item = Location> + '_ {
        self.locations.iter().copied()
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Location> + '_ {
        self.locations
            .iter()
            .flat_map(|location| location.neighbors())
            .filter(move |location| !self.locations.contains(location))
            .collect::<HashSet<_>>() // to make unique
            .into_iter()
    }
}
