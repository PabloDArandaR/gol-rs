use std::fs;
use std::ops::Deref;

use rand::prelude::*;
use rand::distr::StandardUniform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Dead,
    Alive,
}

#[derive(Debug, Clone)]
pub struct Grid{
    cells: Vec<State>,
    height: usize,
    width: usize
}

impl Grid{
    /// Constructor for a Grid instance. Default values for all cells in the new instance are
    /// State::Dead.
    ///
    /// * `height`: total height (number of cells in the y-coordinate)
    /// * `width`: total width (number of cells in the x-coordinate)
    pub fn new(height: usize, width: usize) -> Self{
        Self {
            cells: vec![State::Dead; height*width],
            height: height,
            width: width
        }
    }

    /// Get a mutable reference to an indexed cell. row-major order
    ///
    /// * `x`: x-coordinate (along the width)
    /// * `y`: y-coordinate (along the height)
    pub fn at(&mut self, x: usize, y: usize) -> Option<&mut State> {
        self.cells.get_mut(y*self.width + x - 1)
    }

    /// Generates Grid from file. Structure should consist of the following:
    ///     - 1 line that contains a number which defines the width
    ///     - 1 line that contains a number which defines the height
    ///     - 1 line for each alive cell. Must be CSV, e.g. 100,250
    ///
    /// * `filepath`: Path to the file that contains the map
    pub fn new_from_file(filepath: &std::path::Path) -> Result<Self>{

        let contents = fs::read_to_string(filepath)
            .unwrap_or_else(|error| {
                panic!("Could not read {}: {error}", filepath.display())
            });

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

        while let n = data.next() {
            let mut line = n.expect("Shouldn't happen").trim().to_string().split(",");

            let x_position: usize = line
                .next()
                .expect("")
                .to_string()
                .parse()
                .unwrap();
            let y_position: usize = line
                .next()
                .expect("")
                .to_string()
                .parse()
                .unwrap();

            *output.at(x_position, y_position).unwrap() = State::Alive;
        }

       Ok(output) 
    }

    /// Randomize the grid that calls the function
    pub fn randomize(&mut self){
        for cell in self.cells.iter_mut(){
            let sampled_value: bool = rand::rng().sample(StandardUniform);
            if sampled_value{
                *cell = State::Dead;
            }
            else {
                *cell = State::Alive;
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

    /// Set a new state for a particular cell
    ///
    /// * `x`: x-coordinate (along the width)
    /// * `y`: y-coordinate (along the height)
    /// * `new_state`: The new State value to be set for the cell
    pub fn set_state(&mut self, x: usize, y: usize, new_state: State) {
        *self.at(x, y).unwrap() = new_state;
    }

    pub fn alive_neighbors(&mut self, x:usize, y:usize) -> u8 {
        let mut count: u8 = 0;

        if *self.at(x-1, y-1).unwrap().deref() == State::Alive{ count += 1;}
        if *self.at(x-1, y).unwrap().deref() == State::Alive{ count += 1;}
        if *self.at(x-1, y+1).unwrap().deref() == State::Alive{ count += 1;}
        if *self.at(x, y-1).unwrap().deref() == State::Alive{ count += 1;}
        if *self.at(x, y+1).unwrap().deref() == State::Alive{ count += 1;}
        if *self.at(x+1, y-1).unwrap().deref() == State::Alive{ count += 1;}
        if *self.at(x-1, y).unwrap().deref() == State::Alive{ count += 1;}
        if *self.at(x-1, y+1).unwrap().deref() == State::Alive{ count += 1;}

        count
    }

}
