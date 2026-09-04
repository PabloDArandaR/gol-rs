use std::time::{Duration, Instant};

use crate::graphics::Graphics;
use crate::visual_configuration::VisualsConfiguration;

use gol_backend::game::Game;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

#[derive(Debug)]
pub struct App {
    game: Game,
    graphics: Option<Graphics>,
    visuals_config: VisualsConfiguration,
    tick_rate: Duration,
    next_tick: Instant,
}

impl App {
    pub fn new(_game_config_path: Option<&std::path::Path>) -> Self {
        let visuals_config = VisualsConfiguration::new();
        let game = Game::new();

        Self {
            game: game,
            graphics: None,
            visuals_config: visuals_config,
            tick_rate: Duration::from_millis(100),
            next_tick: Instant::now(),
        }
    }
    pub fn run() {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut app = App::new(None);

        let _ = event_loop.run_app(&mut app);
    }

    fn render(&mut self) {
        self.graphics.as_mut().unwrap().render(&self.visuals_config);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.graphics.is_none() {
            let mut graphics = Graphics::new();
            graphics.init(event_loop);
            graphics.request_redraw();
            self.graphics = Some(graphics);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                println!("New size: {} x {}", size.width, size.height);
                self.graphics.as_mut().unwrap().resize(size);
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_tick {
            let update = self.game.advance();
            if let Some(graphics) = self.graphics.as_mut() {
                graphics.get_buffer_mut().add_game_update(&update);
                graphics.request_redraw();
            }

            self.next_tick = now + self.tick_rate;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_tick));
    }
}
