extern crate sdl2;
extern crate nalgebra_glm as glm;
#[cfg(target_arch = "wasm32")]
extern crate emscripten_main_loop;

use std::rc::Rc;
use std::cell::RefCell;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::GLContext;
use crate::engine::audio_manager::AudioManager;
use crate::engine::drawable_implements::generator::DrawableGenerator;
use super::scene::Scene;
use glm::Mat4;
use glm::ortho;
use std::time::{SystemTime};


#[cfg(target_arch = "wasm32")]
extern "C" {
    fn emscripten_get_now() -> f64;
}

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
    pub width: u32,
    pub height: u32,
    current_scene: Option<Rc<RefCell<dyn Scene>>>,
    pub drawable_generator: DrawableGenerator,
    pub audio_manager: AudioManager<'static>,
    pub current_projection_matrix: Mat4,

    // Native Part
    sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,
    pub sdl2_ttf_context: Rc<Sdl2TtfContext>,
    #[cfg(target_arch = "wasm32")]
    last_updated_time: f64,
    #[cfg(not(target_arch = "wasm32"))]
    last_updated_time: SystemTime,
}

impl Game {
    pub fn new(title: &str, width: u32, height: u32, is_web: bool) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

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

        #[cfg(target_arch = "wasm32")]
        let current_time = unsafe {
            emscripten_get_now()
        };

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
            #[cfg(target_arch = "wasm32")]
            last_updated_time: current_time,
            #[cfg(not(target_arch = "wasm32"))]
            last_updated_time: SystemTime::now(),
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
                    let mut hit_position: Option<(i32, i32)> = None;
                    for event in game.sdl_context.event_pump().unwrap().poll_iter() {
                        match event {
                            sdl2::event::Event::Quit { .. } => return emscripten_main_loop::MainLoopEvent::Terminate,
                            sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                                if game.audio_manager.is_started {
                                    hit_position = Some((x * 1920 / game.width as i32, 1080 - y * 1080 / game.height as i32));
                                }
                                else{
                                    game.audio_manager.start_music();
                                }
                            },
                            sdl2::event::Event::Window { win_event, .. } => {
                                if let sdl2::event::WindowEvent::Resized(window_width, window_height) = win_event {
                                    update_viewport(window_width, window_height);
                                    game.width = window_width as u32;
                                    game.height = window_height as u32;
                                    game.current_projection_matrix = setup_orthographic_projection();
                                }
                            }
                            _ => {}
                        }
                    }

                    game.update(hit_position);
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
                let mut hit_position : Option<(i32, i32)> = None;
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'running,
                        sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                            hit_position = Some((x * 1920 / self.width as i32, 1080 - y * 1080 / self.height as i32));
                        },
                        sdl2::event::Event::Window { win_event, .. } => {
                            if let sdl2::event::WindowEvent::Resized(window_width, window_height) = win_event {
                                update_viewport(window_width, window_height);
                                self.width = window_width as u32;
                                self.height = window_height as u32;
                                self.current_projection_matrix = setup_orthographic_projection();
                            }
                        },
                        _ => {}
                    }
                }

                self.update(hit_position);
                self.draw();
                self.window.gl_swap_window();
            }
        }
    }

    fn update(&mut self, hit_position: Option<(i32, i32)>) {
        #[cfg(target_arch = "wasm32")]
        let current_time = unsafe {
            emscripten_get_now()
        };
        #[cfg(not(target_arch = "wasm32"))]
        let current_time = SystemTime::now();

        #[cfg(target_arch = "wasm32")]
        let delta_time = ((current_time - self.last_updated_time) / 1000.0) as f32;
        #[cfg(not(target_arch = "wasm32"))]
        let delta_time = self.last_updated_time.elapsed().unwrap().as_secs_f32();

        if let Some(scene) = self.current_scene.take() {
            scene.borrow_mut().update(self, delta_time, hit_position);
            self.current_scene = Some(scene);
        }
        self.last_updated_time = current_time;
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

    pub fn start_music(&mut self){
        self.audio_manager.start_music();
    }
}
