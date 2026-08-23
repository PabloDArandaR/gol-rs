use std::{f32, ffi::CString, fs, num::NonZeroU32};

use glow::HasContext;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use gol_backend::game::Game;
use raw_window_handle::HasWindowHandle;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

#[derive(Debug, Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        assert!(r <= 255.0 && r >= 0.0);
        assert!(g <= 255.0 && g >= 0.0);
        assert!(b <= 255.0 && b >= 0.0);
        assert!(a <= 1.0 && a >= 0.0);
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn set_r(&mut self, r: f32) {
        assert!(r <= 255.0 && r >= 0.0);
        self.r = r;
    }

    pub fn set_g(&mut self, g: f32) {
        assert!(g <= 255.0 && g >= 0.0);
        self.g = g;
    }

    pub fn set_b(&mut self, b: f32) {
        assert!(b <= 255.0 && b >= 0.0);
        self.b = b;
    }

    pub fn set_a(&mut self, a: f32) {
        assert!(a <= 1.0 && a >= 0.0);
        self.a = a;
    }
}

#[derive(Debug)]
struct VisualsConfiguration {
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
}

fn load_shader(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read shader")
}

#[derive(Debug)]
struct Graphics {
    window: Option<Window>,
    gl: Option<glow::Context>,
    context: Option<PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,
    program: Option<glow::Program>,
    vao: Option<glow::VertexArray>,
    vbo: Option<glow::Buffer>,
}

impl Graphics {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        // create window
        let window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Test")
                        .with_decorations(true),
                )
                .unwrap(),
        );
        // create glutin context
        // make context current
        // create glow::Context

        // NOW:
        // create VAO
        // create VBO
        // upload vertices
        // compile shaders
        Self {
            window: window,
            gl: Some(gl),
            context: Some(context),
            surface: Some(surface),
            program: Some(surface),
            vao: Some(vao),
            vbo: Some(vbo),
        }
    }
}

#[derive(Debug)]
pub struct App {
    game: Option<Game>,
    window: Option<Window>,
    graphics: Option<Graphics>,
    visuals_config: Option<VisualsConfiguration>,
}

impl App {
    pub fn new() -> Self {
        App {
            game: None,
            window: None,
            graphics: None,
            visuals_config: None,
        }
    }
    pub fn run() {
        let event_loop = EventLoop::new().unwrap();

        let window_attributes = Window::default_attributes()
            .with_title("Test")
            .with_decorations(true);

        let template = ConfigTemplateBuilder::new();

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        // Create the OpenGL context

        let raw_window_hdl = window.window_handle().unwrap().as_raw();

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_hdl));

        let display = gl_config.display();

        let not_current_context = unsafe {
            display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let size = window.inner_size();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_hdl,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );
        let surface = unsafe { display.create_window_surface(&gl_config, &attrs).unwrap() };
        let context = not_current_context.make_current(&surface).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|name| {
                let name = CString::new(name).unwrap();
                display.get_proc_address(&name) as *const _
            })
        };

        let vertices: [f32; 4] = [
            -0.5, 0.0, // point 1
            0.5, 0.0, // point 2
        ];

        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let vertex_bytes = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );

            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertex_bytes, glow::STATIC_DRAW);
        }

        loop {
            unsafe {
                gl.clear_color(0.1, 0.2, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }

            surface.swap_buffers(&context).unwrap();
        }

        //event_loop.set_control_flow(ControlFlow::Poll);

        //event_loop.set_control_flow(ControlFlow::Wait);

        //let mut app = App::default();

        //event_loop.run_app(&mut app);
    }

    fn render(&self) {
        let graphics = self.graphics.as_ref().unwrap();
        let window = graphics.window.as_ref().unwrap();
        let gl = graphics.gl.as_ref().unwrap();
        let context = graphics.context.as_ref().unwrap();
        let surface = graphics.surface.as_ref().unwrap();

        let size = window.inner_size();
    }

    pub fn line_quad(a: [f32; 2], b: [f32; 2], w: f32) -> [f32; 12] {
        [
            a[0] - w / 2.,
            a[1],
            a[0] + w / 2.,
            a[1],
            b[0] - w / 2.,
            b[1],
            a[0] + w / 2.,
            a[1],
            b[0] + w / 2.,
            b[1],
            b[0] - w / 2.,
            b[1],
        ]
    }

    fn draw_line(&self, vertex: [f32; 12]) {
        unsafe {
            let gl = self.graphics.as_ref().unwrap().gl.as_ref().unwrap();

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertex),
                glow::DYNAMIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 2 * 4, 0);
            gl.enable_vertex_attrib_array(0);

            gl.use_program(Some(self.graphics.as_ref().unwrap().program.unwrap()));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }

    fn draw_grid(
        &self,
        gl: &glow::Context,
        size: &PhysicalSize<u32>,
        game: &Game,
        visuals: &VisualsConfiguration,
    ) {
        let grid = game.get_grid().unwrap();
        let width_grid = grid.get_width();
        let height_grid = grid.get_height();

        let n_horizontal_lines = height_grid - 1;
        let n_vertical_lines = width_grid - 1;
        let x_increment = size.width / width_grid as u32;
        let y_increment = size.height / height_grid as u32;

        let mut x_index = x_increment;
        let mut y_index = y_increment;

        let horizontal_line_width = y_increment as f32 * visuals.percentage_cell_size;
        let vertical_line_width = x_increment as f32 * visuals.percentage_cell_size;

        for _ in 0..n_horizontal_lines {
            let start = [y_index as f32, 0.0];
            let end = [y_index as f32, size.width as f32];

            let vertex = Self::line_quad(start, end, horizontal_line_width);
            self.draw_line(vertex);

            y_index += y_increment;
        }
        for _ in 0..n_vertical_lines {
            let start = [x_index as f32, 0.0];
            let end = [x_index as f32, size.width as f32];

            let vertex = Self::line_quad(start, end, vertical_line_width);
            self.draw_line(vertex);

            x_index += x_increment;
        }
    }

    fn draw_square(&mut self, color: Color, center: (f32, f32), width: f32, height: f32) {
        let vertex = [
            center[0] - width / 2.,
            center[1] - height / 2,
            center[0] + width / 2.,
            center[1] - height / 2.,
            center[0] - width / 2.,
            center[1] + height / 2.,
            center[0] + width / 2.,
            center[1] - height / 2.,
            center[0] + width / 2.,
            center[1] + height / 2.,
            center[0] - width / 2.,
            center[1] + height / 2.,
        ];

        unsafe {
            let gl = self.graphics.as_ref().unwrap().gl.as_ref().unwrap();

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertex),
                glow::DYNAMIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 2 * 4, 0);
            gl.enable_vertex_attrib_array(0);

            gl.use_program(Some(self.graphics.as_ref().unwrap().program.unwrap()));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }

    fn draw_alive(&mut self) {
        let graphics = self.graphics.unwrap();
        let window = graphics.window.as_ref().unwrap();
        let size = window.inner_size();

        let grid = self.game.unwrap().get_grid().unwrap();
        let grid_height = grid.get_height();
        let grid_width = grid.get_width();
        let alive_list = grid.get_alive();

        // Get the total size of each of the cells (both vertically and horizontally). Therefore,
        // we just need to multiply by that amount + add half that amount in each direction to get
        // the center
        let x_increment = size.width / width_grid as u32;
        let y_increment = size.height / height_grid as u32;
        for cell in alive_list.iter() {
            // Get the index of the cells and transform it into the real position within the real grid
            let center: (f32, f32) = (
                x_increment * cell.0 + x_increment / 2,
                y_increment * cell.1 + y_increment / 2,
            );

            self.draw_square(
                self.visuals_config.unwrap().alive_color,
                center,
                x_increment * (1 - self.visuals_config.unwrap().percentage_cell_size),
                y_increment * (1 - self.visuals_config.unwrap().percentage_cell_size),
            );
        }
    }

    fn draw_dead(&mut self) {}
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        self.window.as_mut().unwrap().set_resizable(false);
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
                self.window.as_ref().unwrap().request_redraw();
                self.render();
            }
            _ => (),
        }
    }
}
