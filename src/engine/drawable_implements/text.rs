use std::ops::{Index, Range};
use std::path::Path;
use nalgebra_glm::Mat4;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use crate::engine::component::Component;
use crate::engine::drawable_component::DrawableComponent;
use crate::engine::rendering::drawable_object::DrawableObject;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::mesh::Mesh;

pub struct Text {
    pub drawable: DrawableObject,
    pub content: String,
    pub end_range_ratio: f32,

    // static
    font_path: String,
    font_size: u16,
}

impl Text {
    pub fn new(
        left_top: (f32, f32),
        z_index: f32,
        content: &String,
        end_range_ratio: f32,
        color: (f32, f32, f32, f32),
        ttf_context: &sdl2::ttf::Sdl2TtfContext,
        font_path: &str,
        font_size: u16,
        vertex_shader: &str,
        fragment_shader: &str) -> Self {

        let range_end = if end_range_ratio >= 1.0 {
            content.len()
        } else {
            content.char_indices().nth((content.char_indices().count() as f32 * end_range_ratio).trunc() as usize).unwrap().0
        };
        let font = ttf_context.load_font(Path::new(&font_path), font_size).unwrap();
        let surface = font.render(if range_end <= 0 { " " } else { &content[..range_end] })
            .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255))
            .unwrap();

        let text_width = surface.width() as f32;
        let text_height = surface.height() as f32;

        let texture_id = create_texture_from_surface(surface);

        let mesh = Mesh {
            vertices: vec![
                left_top.0, left_top.1, z_index,
                left_top.0 + text_width, left_top.1, z_index,
                left_top.0 + text_width, left_top.1 + text_height, z_index,
                left_top.0, left_top.1 + text_height, z_index,
            ],
            tex_coords: vec![
                0.0, 1.0,
                1.0, 1.0,
                1.0, 0.0,
                0.0, 0.0,
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
        };

        let material = Material::new(vec![
            color.0, color.1, color.2, color.3,
            color.0, color.1, color.2, color.3,
            color.0, color.1, color.2, color.3,
            color.0, color.1, color.2, color.3,
        ], Some(texture_id), vertex_shader, fragment_shader);

        Self {
            drawable: DrawableObject::new(mesh, material),
            content: content.to_string(),
            end_range_ratio,
            font_path: font_path.to_string(),
            font_size,
        }
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        let color = &self.drawable.material.color;
        self.drawable.set_color(vec![
            color[0], color[1], color[2], alpha,
            color[0], color[1], color[2], alpha,
            color[0], color[1], color[2], alpha,
            color[0], color[1], color[2], alpha,
        ]);
    }

    pub fn set_range(&mut self,
                     ttf_context: &sdl2::ttf::Sdl2TtfContext,
                     end_range_ratio: f32){
        let range_end = if end_range_ratio >= 1.0 {
            self.content.len()
        } else {
            self.content.char_indices().nth((self.content.char_indices().count() as f32 * end_range_ratio).trunc() as usize).unwrap().0
        };
        let font = ttf_context.load_font(Path::new(&self.font_path), self.font_size).unwrap();
        let surface = font.render(if range_end <= 0 { " " } else { &self.content[..range_end] })
            .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let text_width = surface.width() as f32;
        let text_height = surface.height() as f32;

        self.end_range_ratio = end_range_ratio;
        self.drawable.set_texture(Some(create_texture_from_surface(surface)));

        let left_top = (self.drawable.mesh.vertices[0], self.drawable.mesh.vertices[1]);
        let z_index = self.drawable.mesh.vertices[2];
        self.drawable.set_vertices(vec![
            left_top.0, left_top.1, z_index,
            left_top.0 + text_width, left_top.1, z_index,
            left_top.0 + text_width, left_top.1 + text_height, z_index,
            left_top.0, left_top.1 + text_height, z_index,
        ]);
    }

    pub fn get_content_char_indices_count(&self) -> usize {
        self.content.char_indices().count()
    }
}

fn create_texture_from_surface(surface: sdl2::surface::Surface) -> u32 {
    let surface = surface.convert_format(sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();
    let width = surface.width();
    let height = surface.height();
    let surface_pixels = surface.without_lock().unwrap();

    let mut texture_id: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            surface_pixels.as_ptr() as *const std::os::raw::c_void,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    texture_id
}

impl Component for Text {
    fn update(&mut self) {
        /* Empty */
    }

    fn as_drawable(&self) -> Option<&dyn DrawableComponent> {
        Some(self)
    }
}

impl DrawableComponent for Text {
    fn draw(&self, projection_matrix: Mat4) {
        self.drawable.draw(projection_matrix);
    }
}
