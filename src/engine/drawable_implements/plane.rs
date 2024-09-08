use nalgebra_glm::Mat4;
use sdl2::image::LoadSurface;
use crate::engine::drawable_component::DrawableComponent;
use crate::engine::component::Component;
use crate::engine::rendering::mesh::Mesh;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::drawable_object::DrawableObject;

pub struct Plane {
    pub drawable: DrawableObject
}

impl Plane {
    pub fn new_from_image(
        rect: (f32, f32, f32, f32),
        z_index: f32,
        color: (f32, f32, f32, f32),
        image_path: Option<&String>,
        vertex_shader: &str,
        fragment_shader: &str) -> Self {
        let mesh = Mesh {
            vertices: vec![
                rect.0, rect.1, z_index,
                rect.0 + rect.2, rect.1, z_index,
                rect.0 + rect.2, rect.1 + rect.3, z_index,
                rect.0, rect.1 + rect.3, z_index,
            ],
            tex_coords: vec![
                0.0, 1.0,
                1.0, 1.0,
                1.0, 0.0,
                0.0, 0.0,
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
        };

        let texture_id = image_path.map(load_texture_from_image);
        let material = Material::new(vec![
                color.0, color.1, color.2, color.3,
                color.0, color.1, color.2, color.3,
                color.0, color.1, color.2, color.3,
                color.0, color.1, color.2, color.3,
            ],
            texture_id,
            vertex_shader,
            fragment_shader);

        Self {
            drawable: DrawableObject::new(mesh, material),
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
}

fn load_texture_from_image(image_path: &String) -> u32 {
    let surface = sdl2::surface::Surface::from_file(image_path).unwrap();
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

impl Component for Plane {
    fn update(&mut self) { /* Empty */ }

    fn as_drawable(&self) -> Option<&dyn DrawableComponent> {
        Some(self)
    }
}

impl DrawableComponent for Plane {
    fn draw(&self, projection_matrix: Mat4) {
        self.drawable.draw(projection_matrix);
    }
}
