use crate::graphics::Graphics;
use crate::visual_configuration::VisualsConfiguration;

use gol_backend::game::Game;
use winit::{application::ApplicationHandler, event::WindowEvent};

#[derive(Debug)]
pub struct App {
    game: Option<Game>,
    graphics: Option<Graphics>,
    visuals_config: Option<VisualsConfiguration>,
}

impl App {
    pub fn new(_game_config_path: Option<&std::path::Path>) -> Self {
        let visuals_config = VisualsConfiguration::new();
        let game = Game::new();

        Self {
            game: Some(game),
            graphics: None,
            visuals_config: Some(visuals_config),
        }
    }
    pub fn run() {
        loop {
            /*
            unsafe {
                gl.clear_color(0.1, 0.2, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }

            surface.swap_buffers(&context).unwrap();
            */
        }

        //event_loop.set_control_flow(ControlFlow::Poll);

        //event_loop.set_control_flow(ControlFlow::Wait);

        //let mut app = App::default();

        //event_loop.run_app(&mut app);
    }

    fn render(&self) {
        // self.graphics.unwrap().render(self.visuals_config);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        /*
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        //self.window.as_mut().unwrap().set_resizable(false);

        if self.graphics == None {
            self.graphics == Graphics::new(self.event_loop);
        }
        */
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                println!("New size: {} x {}", size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                if self.graphics.is_none() {
                    //self.graphics.unwrap().render(&self.visuals_config.unwrap());
                }
                // Add a request_redraw()
                self.graphics
                    .as_mut()
                    .unwrap()
                    .render(&self.visuals_config.as_ref().unwrap());
                self.render();
            }
            _ => (),
        }
    }
}
