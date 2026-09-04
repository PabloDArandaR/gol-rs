use rand::prelude::*;
use std::collections::HashSet;
use std::fs;

use common::cell::CellInstance;
use common::range::ClosedRange;

#[derive(Debug, Clone)]
pub struct GameStateUpdate {
    new_alive: HashSet<CellInstance>,
    new_dead: HashSet<CellInstance>,
}

impl GameStateUpdate {
    pub fn new(new_alive: HashSet<CellInstance>, new_dead: HashSet<CellInstance>) -> Self {
        Self {
            new_alive: new_alive,
            new_dead: new_dead,
        }
    }

    pub fn get_new_alive(&self) -> &HashSet<CellInstance> {
        &self.new_alive
    }

    pub fn get_new_dead(&self) -> &HashSet<CellInstance> {
        &self.new_dead
    }

    pub fn sanitary_check(&self) -> bool {
        for d in self.new_dead.iter() {
            if self.new_alive.contains(d) {
                return false;
            }
        }
        for a in self.new_alive.iter() {
            if self.new_dead.contains(a) {
                return false;
            }
        }
        true
    }

    /// It accumulates the changes from 2 subsequent state updates. If a cell was a new alive cell
    /// and it is now in the input's new dead list, it is just removed from both new_dead and
    /// new_alive. Addition/deletion of cellInstances is not commutative (order matters). If update
    /// 1 has certain alive cell and then it is in the new_dead list of the second update, then the
    ///  final update does not contain the cell, but if it is first dead and then alive, it ends up
    ///  being alive.
    ///
    /// * `update`: new update to be accumulated
    pub fn accumulate_updates(&mut self, update: &Self) -> bool {
        // Remove all the originally new dead elements that are now alive again and viceversa
        if !(self.sanitary_check() && update.sanitary_check()) {
            return false;
        }
        self.new_dead
            .retain(|query| !update.get_new_alive().contains(query));
        self.new_dead.extend(update.get_new_dead());
        self.new_dead
            .retain(|query| update.get_new_alive().contains(query));
        self.new_alive
            .retain(|query| update.get_new_dead().contains(query));

        // Add all the new dead and alive cells
        self.new_dead.extend(update.get_new_dead());
        self.new_alive.extend(update.get_new_alive());

        true
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    alive: HashSet<CellInstance>,
}

impl GameState {
    /// Constructor for a GameState instance.
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
    /// * `n`: ammount of cell instances
    /// * `range`: range of values of the x,y coordinates
    pub fn new_random(n: usize, range: ClosedRange<i32>) -> Self {
        let mut new_grid: Self = Self::new();
        new_grid.randomize(n, range);
        new_grid
    }

    /// Randomize the grid that calls the function
    ///
    /// * `n`: ammount of cell instances
    /// * `range`: range of values of the x,y coordinates
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

    /// Sets the alive cell set given by the input
    ///
    /// * `alive`: the hash set to add
    pub fn set_alive(&mut self, alive: HashSet<CellInstance>) {
        self.alive = alive;
    }
}
