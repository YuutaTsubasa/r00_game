extern crate sdl2;
extern crate nalgebra_glm as glm;
#[cfg(target_arch = "wasm32")]
extern crate emscripten_main_loop;

use std::rc::Rc;
use std::cell::RefCell;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::GLContext;
use crate::engine::audio_manager::AudioManager;
use crate::engine::drawable_implements::generator::DrawableGenerator;
use super::scene::Scene;
use glm::Mat4;
use glm::ortho;


fn setup_orthographic_projection() -> Mat4 {
    ortho(0.0, 1920.0, 0.0, 1080.0, 1.0, -1.0)
}

fn update_viewport(window_width: i32, window_height: i32) {
    unsafe {
        gl::Viewport(0, 0, window_width, window_height);
    }
}

pub struct Game {
    title: String,
    width: u32,
    height: u32,
    current_scene: Option<Rc<RefCell<dyn Scene>>>,
    pub drawable_generator: DrawableGenerator,
    pub audio_manager: AudioManager<'static>,
    pub current_projection_matrix: Mat4,

    // Native Part
    sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,
    sdl2_ttf_context: Rc<Sdl2TtfContext>,
}

impl Game {
    pub fn new(title: &str, width: u32, height: u32, is_web: bool) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // 初始化音頻系統
        sdl2::mixer::init(InitFlag::MP3).unwrap();
        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();
        sdl2::mixer::allocate_channels(4);
        let mut audio_manager = AudioManager::new(is_web);

        let window = video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
        gl_attr.set_context_version(3, 0);

        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        // OpenGL 的設置
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        let sdl2_ttf_context = Rc::new(sdl2::ttf::init().unwrap());
        let drawable_generator = DrawableGenerator::new(Rc::clone(&sdl2_ttf_context));
        let current_projection_matrix = setup_orthographic_projection();

        Self {
            title: title.to_string(),
            width,
            height,
            current_scene: None,
            drawable_generator,
            audio_manager,
            current_projection_matrix,
            sdl_context,
            window,
            gl_context,
            sdl2_ttf_context,
        }
    }

    pub fn run(mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            use emscripten_main_loop::MainLoop;

            struct EngineLoop {
                game: Rc<RefCell<Game>>,
            }

            impl MainLoop for EngineLoop {
                fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
                    let mut game = self.game.borrow_mut();
                    let mut is_hit = false;
                    for event in game.sdl_context.event_pump().unwrap().poll_iter() {
                        match event {
                            sdl2::event::Event::Quit { .. } => return emscripten_main_loop::MainLoopEvent::Terminate,
                            sdl2::event::Event::FingerDown { .. } => {
                                if game.audio_manager.is_started {
                                    is_hit = true;
                                }
                                else{
                                    game.audio_manager.start_music();
                                }
                            },
                            sdl2::event::Event::MouseButtonDown { .. } => {
                                if game.audio_manager.is_started {
                                    is_hit = true;
                                }
                                else{
                                    game.audio_manager.start_music();
                                }
                            },
                            sdl2::event::Event::Window { win_event, .. } => {
                                if let sdl2::event::WindowEvent::Resized(window_width, window_height) = win_event {
                                    update_viewport(window_width, window_height);
                                    game.current_projection_matrix = setup_orthographic_projection();
                                }
                            }
                            _ => {}
                        }
                    }

                    game.update(is_hit);
                    game.draw();
                    game.window.gl_swap_window();
                    emscripten_main_loop::MainLoopEvent::Continue
                }
            }

            let game_rc = Rc::new(RefCell::new(self));
            let app = EngineLoop {
                game: Rc::clone(&game_rc),
            };
            emscripten_main_loop::run(app);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Standalone 的遊戲循環
            let mut event_pump = self.sdl_context.event_pump().unwrap();
            'running: loop {
                let mut is_hit = false;
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'running,
                        sdl2::event::Event::FingerDown { .. } => {
                            is_hit = true;
                        },
                        sdl2::event::Event::MouseButtonDown { .. } => {
                            is_hit = true;
                        },
                        sdl2::event::Event::Window { win_event, .. } => {
                            if let sdl2::event::WindowEvent::Resized(window_width, window_height) = win_event {
                                update_viewport(window_width, window_height);
                                self.current_projection_matrix = setup_orthographic_projection();
                            }
                        },
                        _ => {}
                    }
                }

                self.update(is_hit);
                self.draw();
                self.window.gl_swap_window();
            }
        }
    }

    fn update(&mut self, is_hit: bool) {
        if let Some(scene) = self.current_scene.take() {
            scene.borrow_mut().update(self, is_hit);
            self.current_scene = Some(scene);
        }
    }

    fn draw(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if let Some(scene) = self.current_scene.take() {
            scene.borrow_mut().draw(self);
            self.current_scene = Some(scene);
        }
    }

    pub fn load_scene(&mut self, scene: Rc<RefCell<dyn Scene>>) {
        self.current_scene = Some(scene);
    }

    // pub fn create_plane_from_image(
    //     &self,
    //     rect: (f32, f32, f32, f32),
    //     z_index: f32,
    //     color: (f32, f32, f32, f32),
    //     image_path: Option<&String>,
    //     vertex_shader: &str,
    //     fragment_shader: &str) -> Plane {
    //     self.drawable_generator.generate_plane_from_image(rect, z_index, color, image_path, vertex_shader, fragment_shader)
    // }
    //
    // pub fn create_text(
    //     &self,
    //     left_top: (f32, f32),
    //     z_index: f32,
    //     content: String,
    //     font_path: &str,
    //     font_size: u16,
    //     vertex_shader: &str,
    //     fragment_shader: &str) -> Text {
    //     self.drawable_generator.generate_text(left_top, z_index, content, font_path, font_size, vertex_shader, fragment_shader)
    // }
    //
    // pub fn play_music(&mut self, file_path: &str) {
    //     self.audio_manager.load_music(file_path);
    //     self.audio_manager.play_music();
    // }

    pub fn start_music(&mut self){
        self.audio_manager.start_music();
    }
}
