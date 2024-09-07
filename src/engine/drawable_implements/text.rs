use std::path::Path;
use nalgebra_glm::Mat4;
use crate::engine::component::Component;
use crate::engine::drawable_component::DrawableComponent;
use crate::engine::rendering::drawable_object::DrawableObject;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::mesh::Mesh;

pub struct Text {
    drawable: DrawableObject,
    content: String,
}

impl Text {
    pub fn new(
        left_top: (f32, f32),
        z_index: f32,
        content: &String,
        color: (f32, f32, f32, f32),
        ttf_context: &sdl2::ttf::Sdl2TtfContext,
        font_path: &str,
        font_size: u16,
        vertex_shader: &str,
        fragment_shader: &str) -> Self {

        let font = ttf_context.load_font(Path::new(font_path), font_size).unwrap();
        let surface = font.render(&content)
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
        }
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