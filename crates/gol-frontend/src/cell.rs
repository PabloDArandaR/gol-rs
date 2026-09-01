use glow::Buffer;
use glow::HasContext;
use glow::VertexArray;
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;
use std::vec::Vec;

use crate::color::Color;
use crate::utils;
use crate::utils::load_shader;
use common::cell::CellInstance;

// TODO: Refactor 1: only 1 instance is used in CPU side
// TODO: Refactor 2: we drop any actual CPU memory, and everything is stored in the GPU
/// Data structure that holds all the relevant information for rendering the Cells from a GoL game.
/// When adding a new instance, this is added both to CPU memory (instances and indices fields) and
/// then to the GPU. It holds 2 Vertex Buffer Objects, a instance_vbo that holds the metadata for
/// each specific cell (a CellInstance value) and a mesh_vbo that holds the information of the mesh
/// of the cells. It also holds the Vertex Array Object related to the other 2 VBO.
///
/// * `instances`: Vector that contains all the current CellInstances. It is the CPU-side bucket of
/// active instances.
/// * `indices`: Used to store the index of the vector in which each instance is stored
/// * `instances_vbo`: Holds the metadata of every active instance in the GPU
/// * `mesh_vbo`: Holds the mesh used by cells for rendering
/// * `vao`: VAO that holds each of the VBO of the instance
#[derive(Debug)]
pub struct CellRenderBuffer {
    instances: Vec<CellInstance>,
    indices: HashMap<CellInstance, usize>,
    instances_vbo: glow::Buffer,
    mesh_vbo: glow::Buffer,
    vao: glow::VertexArray,
    gl: Rc<glow::Context>,
}

impl CellRenderBuffer {
    /// Create a new CellRenderBuffer. It creates the VBO for both CellInstances and their mesh,
    /// bind them to the ARRAY_BUFFER VAO and initialize to empty all the other internal data
    /// structures.
    ///
    /// * `gl`: OpenGL context that is being used by the main app
    pub fn new(gl: Rc<glow::Context>) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let mesh_vbo = gl.create_buffer().unwrap();
            let instance_vbo = gl.create_buffer().unwrap();
            gl.bind_vertex_array(Some(vao));

            ///////////////////////////////////////////////////////////////////////////////////
            // Fill the mesh_VBO data
            let mesh_vertices: [f32; 12] = [
                // Top triangles
                -0.5, -0.5, // V1
                0.5, -0.5, // V2
                -0.5, 0.5, // V3
                // Lower triangles
                0.5, -0.5, // V1
                0.5, 0.5, // V2
                -0.5, 0.5, // V3
            ];
            let mesh_bytes = std::slice::from_raw_parts(
                mesh_vertices.as_ptr() as *const u8,
                mesh_vertices.len() * std::mem::size_of::<f32>(),
            );
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(mesh_vbo));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 2 * 4, 0);
            gl.buffer_data_u8_slice(mesh_vbo.0.get(), mesh_bytes, glow::STATIC_DRAW);

            ///////////////////////////////////////////////////////////////////////////////////
            // Fill the instance_VBO data
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instance_vbo));
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::INT, false, 2 * 4, 0);

            gl.vertex_attrib_divisor(0, 0);
            gl.vertex_attrib_divisor(1, 1);

            CellRenderBuffer {
                instances: vec![],
                indices: HashMap::new(),
                instances_vbo: instance_vbo,
                mesh_vbo: mesh_vbo,
                vao: vao,
                gl: gl,
            }
        }
    }

    /// Add a new CellInstance to the CellRenderBuffer instance. It inserts the value in both CPU
    /// and GPU instance
    ///
    /// * `instance`: instance to be inserted
    pub fn add_instance(&mut self, instance: CellInstance) {
        // Check if GPU buffer size (VBO) needs to be incremented
        if self.instances.len() == self.instances.capacity() {
            unsafe {
                self.update_instances_vbo_size();
            }
        }
        self.instances.push(instance);
        let new_index = self.instances.len() - 1;
        self.indices.insert(instance, new_index);
        unsafe {
            self.update_instances(new_index);
        }
    }

    /// Checks if a certain instance is already held by the buffer (by location)
    //
    /// * `instance`: instance to be checked
    pub fn exists_instance(&self, instance: CellInstance) -> bool {
        self.indices.contains_key(&instance)
    }

    /// Deletes a certain instance from the CellRenderBuffer instance.
    ///
    /// * `instance`: instance to be deleted.
    pub fn delete_instance(&mut self, instance: CellInstance) -> bool {
        if self.exists_instance(instance) {
            let removed_index = self.indices.remove(&instance).unwrap();
            let moved = *self.instances.last().unwrap();
            self.instances.swap_remove(removed_index);
            if removed_index < self.instances.len() {
                self.indices.insert(moved, removed_index);
                unsafe {
                    self.update_instances(removed_index);
                }
            }

            return true;
        }
        println!(
            "We are trying to remove an instance that doesn't exist... At {}, {}",
            instance.position[0], instance.position[1]
        );

        false
    }

    /// Updates the value in the buffer to be the one in the self.instances field of the current
    /// CellRenderBuffer instance
    ///
    /// * `index`: index of the relevant instance in the self.instances field
    unsafe fn update_instances(&mut self, index: usize) {
        unsafe {
            let bytes = std::slice::from_raw_parts(
                self.instances.index(index) as *const CellInstance as *const u8,
                std::mem::size_of::<CellInstance>(),
            );

            let offset = index * std::mem::size_of::<CellInstance>();
            self.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.instances_vbo));
            self.gl
                .named_buffer_sub_data_u8_slice(self.instances_vbo, offset as i32, bytes);
        }
    }

    /// Reallocates the VBO with a new memory slot with double the size of the current instances
    /// vector. It doesn't do any internal checks, this should be done outside of the function.
    unsafe fn update_instances_vbo_size(&mut self) {
        unsafe {
            self.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.instances_vbo));
            self.gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                (self.instances.len() * 2 * size_of::<CellInstance>()) as i32,
                glow::DYNAMIC_DRAW,
            );

            let bytes = std::slice::from_raw_parts(
                self.instances.as_ptr() as *const u8,
                self.instances.len() * std::mem::size_of::<CellInstance>(),
            );
            self.gl
                .buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, bytes);
        }
    }

    /// Draws all the instances that are held in this CellRenderBuffer into the given context.
    pub fn draw_instances(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl
                .draw_arrays_instanced(glow::TRIANGLES, 0, 6, self.instances.len() as i32);
        };
    }
}
