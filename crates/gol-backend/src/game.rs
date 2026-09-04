use std::collections::HashSet;

use crate::game_state::{GameState, GameStateUpdate};
use common::cell::CellInstance;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    current_state: GameState,
}

impl Game {
    /// Constructor of a game
    pub fn new() -> Self {
        Game {
            current_state: GameState::new(),
        }
    }

    /// Core function of the Game class. It advances the game 1 iteration. Follows the standard
    /// rules of the game of life
    pub fn advance(&mut self) -> GameStateUpdate {
        let mut new_alive = HashSet::new();
        let mut new_dead = HashSet::new();
        let new_candidate_count = self.get_candidate_count();
        let mut new_alive_list: HashSet<CellInstance> = HashSet::new();
        for candidate in new_candidate_count.iter() {
            if self.current_state.get_alive().contains(candidate.0) {
                if *candidate.1 >= 2 && *candidate.1 <= 3 {
                    new_alive_list.insert(*candidate.0);
                } else {
                    new_dead.insert(*candidate.0);
                }
            } else {
                if *candidate.1 == 3 {
                    new_alive_list.insert(*candidate.0);
                    new_alive.insert(*candidate.0);
                }
            }
        }
        self.current_state.set_alive(new_alive_list);

        GameStateUpdate::new(new_alive, new_dead)
    }

    /// Adcances the game for a given amount of steps
    ///
    /// * `n`: number of steps to advance the game
    pub fn advance_n(&mut self, n: usize) {
        for _ in 1..n {
            self.advance();
        }
    }

    /// Get a hashMap that has candidate CellInstances as keys and the amount of neighbouring cells
    /// that are alive as value
    pub fn get_candidate_count(&self) -> HashMap<CellInstance, usize> {
        let mut output = HashMap::new();
        for cell in self.current_state.get_alive().iter() {
            let relevant_cells = [
                CellInstance::new([cell.position[0] - 1, cell.position[1] - 1]),
                CellInstance::new([cell.position[0] - 1, cell.position[1]]),
                CellInstance::new([cell.position[0] - 1, cell.position[1] + 1]),
                CellInstance::new([cell.position[0], cell.position[1] - 1]),
                CellInstance::new([cell.position[0], cell.position[1] + 1]),
                CellInstance::new([cell.position[0] + 1, cell.position[1] - 1]),
                CellInstance::new([cell.position[0] + 1, cell.position[1]]),
                CellInstance::new([cell.position[0] + 1, cell.position[1] + 1]),
            ];
            for neighbor in relevant_cells {
                if output.contains_key(&neighbor) {
                    *output.get_mut(&neighbor).unwrap() += 1;
                } else {
                    output.insert(neighbor, 1);
                }
            }
        }
        output
    }
}
