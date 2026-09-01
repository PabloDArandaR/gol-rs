use crate::color::Color;

#[derive(Debug)]
pub struct VisualsConfiguration {
    percentage_cell_size: f32,
    alive_color: Color,
    dead_color: Color,
}

impl VisualsConfiguration {
    pub fn new() -> Self {
        VisualsConfiguration {
            percentage_cell_size: 0.1,
            alive_color: Color::new(92.0, 155.0, 39.0, 0.83),
            dead_color: Color::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn set_percentage_cell_size(&mut self, new_value: f32) {
        if new_value <= 1.0 || new_value >= 0.0 {
            self.percentage_cell_size = new_value;
        } else {
            println!("Cannot setup new percentage_cell_size. It should be between 0.0 and 1.0");
        }
    }

    pub fn get_alive(&self) -> Color {
        self.alive_color
    }

    pub fn get_dead(&self) -> Color {
        self.dead_color
    }

    pub fn get_percentage_cell_size(&self) -> f32 {
        self.percentage_cell_size
    }

    pub fn set_alive(&mut self, color: Color) {
        self.alive_color = color
    }

    pub fn set_dead(&mut self, color: Color) {
        self.dead_color = color
    }
}
