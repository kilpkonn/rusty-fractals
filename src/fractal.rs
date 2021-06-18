use std::ffi::CString;

use crate::shader::{Shader, ShaderProgram};

static SQUARE_VERTICES: &'static [f32] = &[
    -1.0, 1.0, 0.0,
    -1.0, -1.0, 0.0,
    1.0, -1.0, 0.0,
    1.0, 1.0, 0.0
];

static DRAW_ORDER: &'static [u16] = &[0, 1, 2, 0, 2, 3];

pub struct Fractal {
    program: ShaderProgram,
}

impl Fractal {
    pub fn new() -> Result<Self, String> {
        let vertex_shader = Shader::from_vert_source(
            &CString::new(include_str!("../assets/vertex.glsl")).unwrap()
        )?;

        let fragment_shader = Shader::from_frag_source(
            &CString::new(include_str!("../assets/mandelbrot.glsl")).unwrap()
        )?;

        // let geometry_shader = Shader::from_frag_source(
        //     &CString::new(include_str!("../assets/bifurcation.geom")).unwrap()
        // )?;

        let program = ShaderProgram::from_shaders(
            &[vertex_shader, fragment_shader,] // geometry_shader]
        )?;
        program.set_used();

        Ok(Fractal { program })
    }

    pub fn draw(&self, mvp_matrix: &[f32], window_size: (u32, u32), time: f32) {
        self.program.set_used();
        let position_handle = self.program.get_attrib_location("vPosition").unwrap();
        let matrix_handle = self.program.get_uniform_location("uMVPMatrix").unwrap();
        let time_handle = self.program.get_uniform_location("uTimeDiff").unwrap();
        let window_size_handle = self.program.get_uniform_location("uWindowSize").unwrap();

        unsafe {
            gl::UniformMatrix4fv(matrix_handle, 1, 0, mvp_matrix.as_ptr());
            gl::Uniform2f(window_size_handle, window_size.0 as f32, window_size.1 as f32);
            gl::Uniform1f(time_handle, time);
            gl::EnableVertexAttribArray(position_handle as u32);
            gl::VertexAttribPointer(
                position_handle as u32, 3,
                gl::FLOAT, gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                SQUARE_VERTICES.as_ptr() as *const gl::types::GLvoid,
            );

            gl::DrawElements(
                gl::TRIANGLES, DRAW_ORDER.len() as i32,
                gl::UNSIGNED_SHORT, DRAW_ORDER.as_ptr() as *const gl::types::GLvoid,
            );

            gl::DisableVertexAttribArray(position_handle as u32);
        }
    }

    pub fn draw_bifurcation(&self, window_size: (u32, u32), time: f32) {
        let mut vertices: Vec<f32> = (0..window_size.0).into_iter()
            .map(|n| -1.0 + 2.0 * n as f32 / window_size.0 as f32)
            .collect();
        self.program.set_used();
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader

            let zoom_handle = self.program.get_uniform_location("zoom").unwrap();
            let zoom_shift_r = self.program.get_uniform_location("shift_r").unwrap();
            let zoom_shift_x = self.program.get_uniform_location("shift_x").unwrap();
            gl::Uniform1f(zoom_handle, 1f32);
            gl::Uniform1f(zoom_shift_r, 1f32);
            gl::Uniform1f(zoom_shift_x, 0.5f32);

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (2 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                vertices.len() as i32
            );
        }
    }
}
