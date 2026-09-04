use std::rc::Rc;
use std::{ffi::CString, num::NonZeroU32};

use glow::HasContext;
use glutin::surface::SwapInterval;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::cell_buffer::CellRenderBuffer;
use crate::utils::load_shader;
use crate::visual_configuration::VisualsConfiguration;
use common::cell::CellInstance;

#[derive(Debug)]
pub struct Graphics {
    window: Option<Window>,
    gl: Option<Rc<glow::Context>>,
    context: Option<PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,
    cell_program: Option<glow::Program>,
    grid_program: Option<glow::Program>,
    cell_buffer: Option<CellRenderBuffer>,
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            window: None,
            gl: None,
            context: None,
            surface: None,
            cell_program: None,
            grid_program: None,
            cell_buffer: None,
        }
    }

    pub fn init(&mut self, event_loop: &ActiveEventLoop) {
        // create windows and GL display
        let window_attributes = Window::default_attributes()
            .with_title("Game of Life")
            .with_decorations(true)
            .with_resizable(false);
        let config_template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24);

        let (window, gl_config) = DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes))
            .build(event_loop, config_template, |configs| {
                configs.max_by_key(|config| config.num_samples()).unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        let gl_display = gl_config.display();

        // create glutin context
        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let not_current_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let size = window.inner_size();

        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );

        let surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .unwrap()
        };
        // make context current
        let context = not_current_context.make_current(&surface).unwrap();

        // VSync
        surface
            .set_swap_interval(&context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
            .ok();

        // create glow::Context
        //
        let gl = Rc::new(unsafe {
            glow::Context::from_loader_function(|name| {
                let name = CString::new(name).unwrap();

                gl_display.get_proc_address(&name)
            })
        });

        unsafe {
            println!("GL version: {}", gl.get_parameter_string(glow::VERSION));
            println!(
                "GLSL version: {}",
                gl.get_parameter_string(glow::SHADING_LANGUAGE_VERSION)
            );
            println!("GL vendor: {}", gl.get_parameter_string(glow::VENDOR));
            println!("GL renderer: {}", gl.get_parameter_string(glow::RENDERER));
        }

        unsafe {
            gl.viewport(0, 0, size.width as i32, size.height as i32);
        }

        // compile shaders
        unsafe {
            // Shaders for the cell
            let cell_program = gl.create_program().unwrap();

            let cell_vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            let cell_vertex_shader_source = load_shader("assets/shaders/basic_cell.vert");
            gl.shader_source(cell_vertex_shader, cell_vertex_shader_source.as_str());
            gl.compile_shader(cell_vertex_shader);
            if !gl.get_shader_compile_status(cell_vertex_shader) {
                panic!(
                    "cell vertex shader failed:\n{}",
                    gl.get_shader_info_log(cell_vertex_shader)
                );
            }

            let cell_frag_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            let cell_frag_shader_source = load_shader("assets/shaders/basic_cell.frag");
            gl.shader_source(cell_frag_shader, cell_frag_shader_source.as_str());
            gl.compile_shader(cell_frag_shader);
            if !gl.get_shader_compile_status(cell_frag_shader) {
                panic!(
                    "cell fragment shader failed:\n{}",
                    gl.get_shader_info_log(cell_frag_shader)
                );
            }

            gl.attach_shader(cell_program, cell_vertex_shader);
            gl.attach_shader(cell_program, cell_frag_shader);
            gl.link_program(cell_program);

            if !gl.get_program_link_status(cell_program) {
                panic!("{}", gl.get_program_info_log(cell_program));
            }

            // Shaders for the grid
            let grid_program = gl.create_program().unwrap();

            let grid_vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            let grid_vertex_shader_source = load_shader("assets/shaders/basic_grid.vert");
            gl.shader_source(grid_vertex_shader, grid_vertex_shader_source.as_str());
            gl.compile_shader(grid_vertex_shader);

            let grid_frag_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            let grid_frag_shader_source = load_shader("assets/shaders/basic_grid.frag");
            gl.shader_source(grid_frag_shader, grid_frag_shader_source.as_str());
            gl.compile_shader(grid_frag_shader);

            gl.attach_shader(grid_program, grid_vertex_shader);
            gl.attach_shader(grid_program, grid_frag_shader);
            gl.link_program(grid_program);

            if !gl.get_program_link_status(grid_program) {
                panic!("{}", gl.get_program_info_log(grid_program));
            }

            let cell_buffer = CellRenderBuffer::new(gl.clone());

            self.window = Some(window);
            self.gl = Some(gl);
            self.context = Some(context);
            self.surface = Some(surface);
            self.cell_program = Some(cell_program);
            self.grid_program = Some(grid_program);
            self.cell_buffer = Some(cell_buffer);

            println!("Finished initialization");
        }
    }

    pub fn render(&mut self, config: &VisualsConfiguration) {
        unsafe {
            self.gl.as_mut().unwrap().clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.as_mut().unwrap().clear(glow::COLOR_BUFFER_BIT);

            self.gl.as_mut().unwrap().use_program(self.cell_program);
            let alive_color = config.get_alive();
            let alive_color_location = self
                .gl
                .as_mut()
                .unwrap()
                .get_uniform_location(self.cell_program.unwrap(), "alive_color");

            self.gl.as_mut().unwrap().uniform_4_f32(
                alive_color_location.as_ref(),
                alive_color.r,
                alive_color.g,
                alive_color.b,
                alive_color.a,
            );
            self.cell_buffer.as_mut().unwrap().draw_instances();

            // TODO: Draw the grid itself
        }

        // TODO: ADD Result handling
        let _ = self
            .surface
            .as_ref()
            .unwrap()
            .swap_buffers(self.context.as_ref().unwrap());
    }

    pub fn update_alive_cell_buffer(
        &mut self,
        new_dead: &Vec<CellInstance>,
        new_alive: &Vec<CellInstance>,
    ) {
        new_dead.into_iter().for_each(|value| {
            let _ = self.cell_buffer.as_mut().unwrap().delete_instance(*value);
        });
        new_alive
            .into_iter()
            .for_each(|value| self.cell_buffer.as_mut().unwrap().add_instance(*value));
    }

    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    pub fn get_buffer_mut(&mut self) -> &mut CellRenderBuffer {
        self.cell_buffer.as_mut().unwrap()
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        let Some(surface) = &self.surface else { return };
        let Some(context) = &self.context else { return };
        let Some(gl) = &self.gl else { return };

        surface.resize(
            context,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );

        unsafe {
            gl.viewport(0, 0, size.width as i32, size.height as i32);
        }
    }
}
impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe {
            if let Some(gl) = self.gl.as_mut() {
                gl.delete_program(*self.cell_program.as_mut().unwrap());
                gl.delete_program(*self.grid_program.as_mut().unwrap());
            }
        }
    }
}
