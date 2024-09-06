use super::mesh::Mesh;
use super::material::Material;

pub struct DrawableObject {
    mesh: Mesh,
    material: Material,
}

impl DrawableObject {
    pub fn new(mesh: Mesh, material: Material) -> Self {
        Self {
            mesh,
            material
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.material.shader_program);

            let mut vao: u32 = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut vbo: u32 = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.mesh.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            let mut cbo: u32 = 0;
            gl::GenBuffers(1, &mut cbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, cbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.material.color.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.material.color.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            let mut tbo: u32 = 0;
            gl::GenBuffers(1, &mut tbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, tbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.mesh.tex_coords.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.mesh.tex_coords.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(2);

            gl::DrawElements(gl::TRIANGLES, self.mesh.indices.len() as i32, gl::UNSIGNED_INT, self.mesh.indices.as_ptr() as *const _);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

