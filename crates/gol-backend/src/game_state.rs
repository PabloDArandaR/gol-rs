use rand::prelude::*;
use std::collections::HashSet;
use std::fs;

use common::cell::CellInstance;
use common::range::ClosedRange;

#[derive(Debug, Clone)]
pub struct GameState {
    alive: HashSet<CellInstance>,
}

impl GameState {
    /// Constructor for a Grid instance. Default values for all cells in the new instance are
    /// State::Dead.
    ///
    /// * `height`: total height (number of cells in the y-coordinate)
    /// * `width`: total width (number of cells in the x-coordinate)
    pub fn new() -> Self {
        Self {
            alive: HashSet::new(),
        }
    }

    /// Generates Grid from file. Structure should consist of the following:
    ///     - 1 line for each alive cell. Must be CSV, e.g. 100,250
    ///
    /// * `filepath`: Path to the file that contains the map
    pub fn new_from_file(filepath: &std::path::Path) -> Self {
        let contents = fs::read_to_string(filepath)
            .unwrap_or_else(|error| panic!("Could not read {}: {error}", filepath.display()));

        let data = contents.lines();

        let mut output = Self::new();

        for n in data {
            let mut line = n.trim().split(",");

            let x: i32 = line.next().expect("").to_string().parse().unwrap();
            let y: i32 = line.next().expect("").to_string().parse().unwrap();
            let new_cell = CellInstance::new([x, y]);

            output.insert_alive(&new_cell);
        }

        output
    }

    /// Generate a new random map with a given size
    ///
    /// * `height`: total height (number of cells in the y-coordinate)
    /// * `width`: total width (number of cells in the x-coordinate)
    pub fn new_random(n: usize, range: ClosedRange<i32>) -> Self {
        let mut new_grid: Self = Self::new();
        new_grid.randomize(n, range);
        new_grid
    }

    /// Randomize the grid that calls the function
    fn randomize(&mut self, n: usize, range: ClosedRange<i32>) {
        self.clear();
        for _ in 0..n {
            loop {
                let x = rand::rng().random_range(range.min..range.max);
                let y = rand::rng().random_range(range.min..range.max);
                if self.insert_alive(&CellInstance::new([x, y])) {
                    break;
                }
            }
        }
    }

    /// Insert an alive cell at x,y coordinate
    ///
    /// * `cell`: CellInstance to be added
    pub fn insert_alive(&mut self, cell: &CellInstance) -> bool {
        if !self.alive.contains(cell) {
            self.alive.insert(*cell);
            return true;
        }
        false
    }

    /// Delete an alive cell
    ///
    /// * `cell`: cell to be deleted (should be checked by value)
    pub fn delete_alive(&mut self, cell: &CellInstance) -> bool {
        if self.alive.contains(cell) {
            self.alive.remove(cell);
            return true;
        }
        false
    }

    /// Check if there is an alive cell at certain x,y coordinate
    ///
    /// * `cell`: cell to be checked (should be checked by value)
    pub fn is_alive(&self, cell: &CellInstance) -> bool {
        self.alive.contains(cell)
    }

    /// Clear all the alive cells.
    pub fn clear(&mut self) {
        self.alive = HashSet::new();
    }

    /// Add a reference to the set of alive cells
    pub fn get_alive(&self) -> &HashSet<CellInstance> {
        &self.alive
    }

    pub fn set_alive(&mut self, alive: HashSet<CellInstance>) {
        self.alive = alive;
    }
}
