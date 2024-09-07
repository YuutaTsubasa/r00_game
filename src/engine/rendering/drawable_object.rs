use nalgebra_glm::Mat4;
use crate::engine::drawable_implements::plane::Plane;
use super::mesh::Mesh;
use super::material::Material;

pub struct DrawableObject {
    mesh: Mesh,
    material: Material,
    vao: u32,
    vbo: u32,
    cbo: u32,
    tbo: u32,
    ebo: u32,
}

impl DrawableObject {
    pub fn new(mesh: Mesh, material: Material) -> Self {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut cbo: u32 = 0;
        let mut tbo: u32 = 0;
        let mut ebo: u32 = 0;

        unsafe {
            // 生成並綁定 VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // 頂點緩衝區 (VBO)
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // 顏色緩衝區 (CBO)
            gl::GenBuffers(1, &mut cbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, cbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (material.color.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                material.color.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            // 紋理緩衝區 (TBO)
            gl::GenBuffers(1, &mut tbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, tbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.tex_coords.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                mesh.tex_coords.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(2);

            // 索引緩衝區 (EBO)
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                mesh.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // 解除 VAO 綁定
            gl::BindVertexArray(0);
        }

        Self {
            mesh,
            material,
            vao,
            vbo,
            cbo,
            tbo,
            ebo,
        }
    }

    pub fn draw(&self, projection_matrix: Mat4) {
        unsafe {
            gl::UseProgram(self.material.shader_program);
            gl::BindVertexArray(self.vao);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.material.texture_id.unwrap());

            let uniform_location = unsafe {
                gl::GetUniformLocation(self.material.shader_program, "uProjection\0".as_ptr() as *const i8)
            };
            gl::UniformMatrix4fv(
                uniform_location,
                1,
                gl::FALSE,
                projection_matrix.as_ptr(),
            );

            gl::DrawElements(gl::TRIANGLES, self.mesh.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());

            gl::BindVertexArray(0);
        }
    }
}

impl Drop for DrawableObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.cbo);
            gl::DeleteBuffers(1, &self.tbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteTextures(1, &self.material.texture_id.unwrap());
        }

    }
}

