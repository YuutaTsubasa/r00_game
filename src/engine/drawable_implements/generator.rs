use sdl2::ttf::Sdl2TtfContext;
use crate::engine::drawable_implements::plane::Plane;
use crate::engine::drawable_implements::text::Text;
use std::rc::Rc;

pub struct DrawableGenerator {
    ttf_context: Rc<Sdl2TtfContext>,
}

impl DrawableGenerator {
    pub fn new(ttf_context: Rc<Sdl2TtfContext>) -> Self {
        Self { ttf_context }
    }

    pub fn generate_plane_from_image(
        &self,
        rect: (f32, f32, f32, f32),
        z_index: f32,
        color: (f32, f32, f32, f32),
        image_path: Option<&String>,
        vertex_shader: &str,
        fragment_shader: &str) -> Plane {
        Plane::new_from_image(rect, z_index, color, image_path, vertex_shader, fragment_shader)
    }

    pub fn generate_text(
        &self,
        left_top: (f32, f32),
        z_index: f32,
        content: &String,
        color: (f32, f32, f32, f32),
        font_path: &str,
        font_size: u16,
        vertex_shader: &str,
        fragment_shader: &str) -> Text {
        Text::new(
            left_top,
            z_index,
            content,
            color,
            &self.ttf_context,
            font_path,
            font_size,
            vertex_shader,
            fragment_shader)
    }
}