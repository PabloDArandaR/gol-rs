use rand::prelude::*;
use std::fs;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Grid {
    alive: HashSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Grid {
    /// Constructor for a Grid instance. Default values for all cells in the new instance are
    /// State::Dead.
    ///
    /// * `height`: total height (number of cells in the y-coordinate)
    /// * `width`: total width (number of cells in the x-coordinate)
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            alive: HashSet::new(),
            height: height,
            width: width,
        }
    }

    /// Insert an alive cell at x,y coordinate
    ///
    /// * `x`: x-coordinate (along the width)
    /// * `y`: y-coordinate (along the height)
    pub fn insert_alive(&mut self, x: usize, y: usize) -> bool {
        assert!(x >= 0 && x <= self.width);
        assert!(y >= 0 && y <= self.height);
        if !self.alive.contains(&(x, y)) {
            self.alive.insert((x, y));
            return true;
        }
        false
    }

    /// Check if there is an alive cell at certain x,y coordinate
    ///
    /// * `x`: x-coordinate (along the width)
    /// * `y`: y-coordinate (along the height)
    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.alive.contains(&(x, y))
    }

    /// Generates Grid from file. Structure should consist of the following:
    ///     - 1 line that contains a number which defines the width
    ///     - 1 line that contains a number which defines the height
    ///     - 1 line for each alive cell. Must be CSV, e.g. 100,250
    ///
    /// * `filepath`: Path to the file that contains the map
    pub fn new_from_file(filepath: &std::path::Path) -> Self {
        let contents = fs::read_to_string(filepath)
            .unwrap_or_else(|error| panic!("Could not read {}: {error}", filepath.display()));

        let mut data = contents.lines();

        let width: usize = data
            .next()
            .expect("Wrong file: no header found")
            .trim()
            .parse()
            .expect("Wrong file: width is not a valid positive integer");

        let height: usize = data
            .next()
            .expect("Wrong file: incomplete header")
            .trim()
            .parse()
            .expect("Wrong file: height is not a valid positive integer");

        let mut output = Self::new(width, height);

        for n in data {
            let mut line = n.trim().split(",");

            let x: usize = line.next().expect("").to_string().parse().unwrap();
            let y: usize = line.next().expect("").to_string().parse().unwrap();

            output.insert_alive(x, y);
        }

        output
    }

    /// Clear all the alive cells.
    pub fn clear(&mut self) {
        self.alive = HashSet::new();
    }

    /// Randomize the grid that calls the function
    pub fn randomize(&mut self) {
        self.clear();
        let n = rand::rng().random_range(0..self.width * self.height);
        for _ in 0..n {
            loop {
                let x = rand::rng().random_range(0..self.width);
                let y = rand::rng().random_range(0..self.height);
                if self.insert_alive(x, y) {
                    break;
                }
            }
        }
    }

    /// Generate a new random map with a given size
    ///
    /// * `height`: total height (number of cells in the y-coordinate)
    /// * `width`: total width (number of cells in the x-coordinate)
    pub fn new_random(height: usize, width: usize) -> Self {
        let mut new_grid: Self = Self::new(height, width);
        new_grid.randomize();
        new_grid
    }

    /// Get the total height of the instance
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Get the total width of the instance
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Check if a x,y coordinatea is within the grid
    ///
    /// * `x`: x-coordinate (along the width)
    /// * `y`: y-coordinate (along the height)
    fn is_inside(&self, x: usize, y: usize) -> bool {
        x <= self.width && y <= self.height
    }

    /// List all the neighbors to alive cells
    pub fn list_all_neighbors(&self) -> HashSet<(usize, usize)> {
        let mut neighbors = HashSet::new();

        for cell in self.alive.iter() {
            let relevant_cells = [
                (cell.0 - 1, cell.1 - 1),
                (cell.0 - 1, cell.1),
                (cell.0 - 1, cell.1 + 1),
                (cell.0, cell.1 - 1),
                (cell.0, cell.1 + 1),
                (cell.0 + 1, cell.1 - 1),
                (cell.0 + 1, cell.1),
                (cell.0 + 1, cell.1 + 1),
            ];
            for rel in relevant_cells {
                if !self.is_alive(rel.0, rel.1) && self.is_inside(rel.0, rel.1) {
                    neighbors.insert(rel);
                }
            }
        }

        neighbors
    }

    /// Add a reference to the set of alive cells
    pub fn get_alive(&self) -> &HashSet<(usize, usize)> {
        &self.alive
    }

    pub fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count: usize = 0;
        let relevant_cells = [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ];

        for cell in relevant_cells.iter() {
            if self.is_alive(cell.0, cell.1) {
                count += 1;
            }
        }

        count
    }

    pub fn set_alive(&mut self, alive: HashSet<(usize, usize)>) {
        self.alive = alive;
    }
}
