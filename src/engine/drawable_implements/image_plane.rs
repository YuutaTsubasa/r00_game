use sdl2::image::LoadSurface;
use crate::engine::drawable_component::DrawableComponent;
use crate::engine::component::Component;
use crate::engine::rendering::mesh::Mesh;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::drawable_object::DrawableObject;

pub struct ImagePlane {
    drawable: DrawableObject
}

impl ImagePlane {
    pub fn new(
        rect: (f32, f32, f32, f32),
        color: (f32, f32, f32, f32),
        image_path: Option<&str>,
        vertex_shader: &str,
        fragment_shader: &str) -> Self {
        let mesh = Mesh {
            vertices: vec![
                rect.0, rect.1, 0.0,
                rect.0 + rect.2, rect.1, 0.0,
                rect.0 + rect.2, rect.1 + rect.3, 0.0,
                rect.0, rect.1 + rect.3, 0.0,
            ],
            tex_coords: vec![
                0.0, 0.0,  // 左上
                1.0, 0.0,  // 右上
                1.0, 1.0,  // 右下
                0.0, 1.0,  // 左下
            ],
            indices: vec![0, 1, 2, 2, 3, 0], // 兩個三角形構成矩形
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
}

fn load_texture_from_image(image_path: &str) -> u32 {
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

impl Component for ImagePlane {
    fn update(&mut self) { /* Empty */ }

    fn as_drawable(&self) -> Option<&dyn DrawableComponent> {
        Some(self)
    }
}

impl DrawableComponent for ImagePlane {
    fn draw(&self) {
        self.drawable.draw();
    }
}