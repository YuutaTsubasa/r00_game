// extern crate sdl2;
// extern crate gl;
// #[cfg(target_arch = "wasm32")]
// extern crate emscripten_main_loop;
//
// use sdl2::ttf::Sdl2TtfContext;
// use sdl2::image::{InitFlag, LoadSurface};
// use sdl2::surface::Surface;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::mixer::{Music, AUDIO_S16LSB, DEFAULT_CHANNELS};
// use std::ffi::CString;
// use std::path::Path;
// use std::rc::Rc;
//
// struct TextRenderer<'a> {
//     font: sdl2::ttf::Font<'a, 'a>,
// }
//
// impl<'a> TextRenderer<'a> {
//     fn new(ttf_context: &'a Sdl2TtfContext, font_path: &str, font_size: u16) -> Self {
//         let font = ttf_context.load_font(Path::new(font_path), font_size).unwrap();
//         Self { font }
//     }
//
//     fn render_text(&self, text: &str) -> Surface<'_> {
//         let surface = self.font
//             .render(text)
//             .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255))
//             .unwrap();
//         surface
//     }
// }
//
// fn initialize_audio() {
//     sdl2::mixer::init(sdl2::mixer::InitFlag::MP3).unwrap();
//     sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();
//     sdl2::mixer::allocate_channels(4);
// }
//
// fn play_music(file_path: &str) -> Music<'static> {
//     let music = sdl2::mixer::Music::from_file(file_path).unwrap();
//     music.play(-1).unwrap();
//     music
// }
//
// #[no_mangle]
// pub extern "C" fn start_music() -> Music<'static> {
//     initialize_audio();
//     play_music("./resources/background.mp3")
// }
//
// fn initialize_sdl_and_opengl() -> (
//     sdl2::Sdl,
//     sdl2::video::Window,
//     sdl2::video::GLContext) {
//     let sdl = sdl2::init().unwrap();
//     let video_subsystem = sdl.video().unwrap();
//
//     let window = video_subsystem.window("AVG Game", 800, 600)
//         .opengl()
//         .resizable()
//         .position_centered()
//         .build()
//         .unwrap();
//
//     let gl_attr = video_subsystem.gl_attr();
//     gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
//     gl_attr.set_context_version(3, 0);
//
//     let gl_context = window.gl_create_context().unwrap();
//     window.gl_make_current(&gl_context).unwrap();
//     let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
//
//     unsafe {
//         gl::Enable(gl::DEPTH_TEST);
//         gl::Enable(gl::BLEND);
//         gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
//         gl::Viewport(0, 0, 800, 600);
//         gl::ClearColor(0.0, 0.0, 0.0, 1.0);
//     }
//
//     (sdl, window, gl_context)
// }
//
// fn draw_frame(texture_id: u32, text_renderer: &TextRenderer) {
//     unsafe {
//         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
//         draw_triangle(texture_id);
//         draw_text(&text_renderer);
//     }
// }
//
// fn draw_text(text_renderer: &TextRenderer) {
//     let surface = text_renderer.render_text("悠太翼");
//     let surface = surface.convert_format(PixelFormatEnum::RGBA32).unwrap();
//     let width = surface.width();
//     let height = surface.height();
//     let surface_pixels = surface.without_lock().unwrap();
//
//     let mut texture_id: u32 = 0;
//     unsafe {
//         gl::GenTextures(1, &mut texture_id);
//         gl::BindTexture(gl::TEXTURE_2D, texture_id);
//         gl::TexImage2D(
//             gl::TEXTURE_2D,
//             0,
//             gl::RGBA as i32,
//             width as i32,
//             height as i32,
//             0,
//             gl::RGBA,
//             gl::UNSIGNED_BYTE,
//             surface_pixels.as_ptr() as *const std::os::raw::c_void);
//
//         // 設置紋理參數
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
//     }
//
//     let vertices: [f32; 48] = [
//         // 第一個三角形
//         -0.75,  0.0, -0.1, 1.0, 1.0, 1.0, 0.0, 0.0,  // 左上
//         0.75,  0.0, -0.1, 1.0, 1.0, 1.0, 1.0, 0.0,  // 右上
//         0.75, -0.75, -0.1, 1.0, 1.0, 1.0, 1.0, 1.0, // 右下
//
//         // 第二個三角形
//         -0.75,  0.0, -0.1, 1.0, 1.0, 1.0, 0.0, 0.0,  // 左上
//         0.75, -0.75, -0.1, 1.0, 1.0, 1.0, 1.0, 1.0, // 右下
//         -0.75, -0.75, -0.1, 1.0, 1.0, 1.0, 0.0, 1.0  // 左下
//     ];
//
//     let mut vao: u32 = 0;
//     let mut vbo: u32 = 0;
//     let shader_program = create_shader_program();
//     let shader_texture1_name = CString::new("texture1").unwrap();
//
//     unsafe {
//         gl::GenVertexArrays(1, &mut vao);
//         gl::BindVertexArray(vao);
//
//         gl::GenBuffers(1, &mut vbo);
//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         gl::BufferData(
//             gl::ARRAY_BUFFER,
//             (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
//             vertices.as_ptr() as *const gl::types::GLvoid,
//             gl::STATIC_DRAW);
//
//         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, std::ptr::null());
//         gl::EnableVertexAttribArray(0);
//
//         gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
//         gl::EnableVertexAttribArray(1);
//
//         gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
//         gl::EnableVertexAttribArray(2);
//
//         gl::UseProgram(shader_program);
//
//         gl::ActiveTexture(gl::TEXTURE0);
//         gl::BindTexture(gl::TEXTURE_2D, texture_id);
//         gl::Uniform1i(gl::GetUniformLocation(shader_program, shader_texture1_name.as_ptr()), 0);
//         gl::DrawArrays(gl::TRIANGLES, 0, 6);  // 繪製 6 個頂點，形成兩個三角形
//     }
// }
//
// fn load_texture_from_image_file(file_path: &str) -> u32 {
//     sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
//
//     let surface = Surface::from_file(Path::new(file_path)).unwrap();
//     let surface = surface.convert_format(PixelFormatEnum::RGBA32).unwrap();
//     let width = surface.width();
//     let height = surface.height();
//     let surface_pixels = surface.without_lock().unwrap();
//
//     let mut texture_id: u32 = 0;
//     unsafe {
//         gl::GenTextures(1, &mut texture_id);
//         gl::BindTexture(gl::TEXTURE_2D, texture_id);
//         gl::TexImage2D(
//             gl::TEXTURE_2D,
//             0,
//             gl::RGBA as i32,
//             width as i32,
//             height as i32,
//             0,
//             gl::RGBA,
//             gl::UNSIGNED_BYTE,
//             surface_pixels.as_ptr() as *const std::os::raw::c_void);
//
//         // 設置紋理參數
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
//         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
//     }
//
//     texture_id
// }
//
// #[cfg(not(target_arch = "wasm32"))]
// fn main() {
//     let (sdl, window, _gl_context) = initialize_sdl_and_opengl();
//
//     // 加載紋理
//     let texture_id = load_texture_from_image_file("./resources/texture1.png");
//     let _music = start_music();
//     let ttf_context = sdl2::ttf::init().unwrap();
//     let text_renderer = TextRenderer::new(&ttf_context, "./resources/SourceHanSerifTC-Heavy.otf", 24);
//
//     let mut event_pump = sdl.event_pump().unwrap();
//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 sdl2::event::Event::Quit {..} => {
//                     break 'running
//                 },
//                 _ => {}
//             }
//         }
//
//         draw_frame(texture_id, &text_renderer);
//         window.gl_swap_window();
//     }
// }
//
// #[cfg(target_arch = "wasm32")]
// fn main() {
//     use emscripten_main_loop::MainLoop;
//
//     struct MyApp {
//         window: sdl2::video::Window,
//         texture_id: u32,
//         ttf_context: Sdl2TtfContext
//     }
//
//     impl MainLoop for MyApp {
//         fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
//             let text_renderer = TextRenderer::new(&self.ttf_context, "./resources/SourceHanSerifTC-Heavy.otf", 24);
//             draw_frame(self.texture_id, &text_renderer);
//             self.window.gl_swap_window();
//             emscripten_main_loop::MainLoopEvent::Continue
//         }
//     }
//
//     let (_sdl, window, _gl_context) = initialize_sdl_and_opengl();
//
//     let texture_id = load_texture_from_image_file("./resources/texture1.png");
//     let ttf_context = sdl2::ttf::init().unwrap();
//
//     let app = MyApp { window, texture_id, ttf_context };
//     emscripten_main_loop::run(app);
// }
//
// fn draw_triangle(texture_id: u32) {
//     let vertices: [f32; 24] = [
//         0.0, 0.5, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0,
//         -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
//         0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0
//     ];
//
//     let mut vao: u32 = 0;
//     let mut vbo: u32 = 0;
//     let shader_program = create_shader_program();
//     let shader_texture1_name = CString::new("texture1").unwrap();
//
//     unsafe {
//         gl::GenVertexArrays(1, &mut vao);
//         gl::BindVertexArray(vao);
//
//         gl::GenBuffers(1, &mut vbo);
//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         gl::BufferData(
//             gl::ARRAY_BUFFER,
//             (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
//             vertices.as_ptr() as *const gl::types::GLvoid,
//             gl::STATIC_DRAW);
//
//         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, std::ptr::null());
//         gl::EnableVertexAttribArray(0);
//
//         gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
//         gl::EnableVertexAttribArray(1);
//
//         gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
//         gl::EnableVertexAttribArray(2);
//
//         gl::UseProgram(shader_program);
//
//         gl::ActiveTexture(gl::TEXTURE0);
//         gl::BindTexture(gl::TEXTURE_2D, texture_id);
//         gl::Uniform1i(gl::GetUniformLocation(shader_program, shader_texture1_name.as_ptr()), 0);
//         gl::DrawArrays(gl::TRIANGLES, 0, 3);
//     }
// }
//

mod engine;

use engine::game::Game;
use engine::scene::Scene;
use engine::drawable_implements::text::Text;
use engine::drawable_implements::image_plane::ImagePlane;

fn main() {
    let mut game = Game::new("AVG Game", 800, 600);

    let image_plane = ImagePlane::new(
        (-0.5, -0.5, 1.0, 1.0),
        (1.0, 1.0, 1.0, 1.0),
        Some("./resources/texture1.png"),
        include_str!("shaders/vertex_shader.glsl"),
        include_str!("shaders/fragment_shader.glsl"));

    let ttf_context = sdl2::ttf::init().unwrap();

    let text = Text::new(
        (-0.75,  0.0),
        "悠太翼".to_string(),
        &ttf_context,
        "./resources/SourceHanSerifTC-Heavy.otf",
        24,
        include_str!("shaders/vertex_shader.glsl"),
        include_str!("shaders/fragment_shader.glsl"),
    );

    let mut scene = Scene::new();
    scene.add_component(Box::new(text));
    scene.add_component(Box::new(image_plane));

    game.load_scene(scene);
    game.run();
}