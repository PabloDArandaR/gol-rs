use crate::limited_grid_map::{Grid, State};

pub struct Game{
    grid: Option<Box<Grid>>
}

impl Game {

    pub fn new() -> Self{
        Game { grid: None }
    }

    pub fn set_grid(&mut self, grid: Box<Grid>) {
        self.grid = Some(grid);
    }

    /// Core function of the Game class. It advances the game 1 iteration. Follows the standard
    /// rules of the game of life
    pub fn advance(&mut self) {
        for i in 1..self.grid.unwrap().get_width(){
            for j in 1..self.grid.unwrap().get_height() {
                let n_alive = self.grid.unwrap().alive_neighbors(i, j);
                let grid_cel_ref = self.grid.unwrap().at(i, j).unwrap();
                if *grid_cel_ref == State::Alive{
                    if n_alive < 2 || n_alive > 3{
                        grid_cel_ref = State::Dead; 
                    }
                }
                else if *grid_cel_ref == State::Dead {
                    if n_alive == 3 {
                        grid_cel_ref = State::Alive;
                    }
                }
            }
        }
    }

    pub fn advance_n(&mut self, n: usize) {
        for _ in 1..n{
            self.advance();
        }
    }

    pub fn get_grid(&self) -> Option<Grid>{
        self.grid.as_deref().cloned()
    }
}
