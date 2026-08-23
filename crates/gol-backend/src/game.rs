use std::collections::HashSet;

use crate::limited_grid_map::Grid;

#[derive(Debug)]
pub struct Game {
    grid: Option<Box<Grid>>,
}

impl Game {
    pub fn new() -> Self {
        Game { grid: None }
    }

    pub fn set_grid(&mut self, grid: Box<Grid>) {
        self.grid = Some(grid);
    }

    /// Core function of the Game class. It advances the game 1 iteration. Follows the standard
    /// rules of the game of life
    pub fn advance(&mut self) {
        let grid = self.get_grid().unwrap();
        let alive_list = grid.get_alive();
        let neighbors_list = self.get_grid().unwrap().list_all_neighbors();
        let mut new_alive_list: HashSet<(usize, usize)> = HashSet::new();
        for cell in neighbors_list.iter() {
            if self
                .get_grid()
                .unwrap()
                .count_alive_neighbors(cell.0, cell.1)
                == 3 as usize
            {
                new_alive_list.insert(*cell);
            }
        }
        for cell in alive_list.iter() {
            let n_alive_neighbors = self
                .get_grid()
                .unwrap()
                .count_alive_neighbors(cell.0, cell.1);
            if n_alive_neighbors == 2 || n_alive_neighbors == 3 {
                new_alive_list.insert(*cell);
            }
        }
        let mut grid = self.get_grid().unwrap();
        grid.set_alive(new_alive_list);
    }

    pub fn advance_n(&mut self, n: usize) {
        for _ in 1..n {
            self.advance();
        }
    }

    pub fn get_grid(&self) -> Option<Grid> {
        self.grid.as_deref().cloned()
    }
}
