extern crate sdl2;
#[cfg(target_arch = "wasm32")]
extern crate emscripten_main_loop;

use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::video::GLContext;
use super::scene::Scene;

pub struct Game {
    title: String,
    width: u32,
    height: u32,
    current_scene: Option<Scene>,
    game_native_context: GameNativeContext,
}

struct GameNativeContext {
    sdl_context: Sdl,
    window: Window,
    gl_context: GLContext
}

impl Game {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

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
            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        let game_native_context = GameNativeContext {
            sdl_context,
            window,
            gl_context
        };

        Self {
            title: title.to_string(),
            width,
            height,
            current_scene: None,
            game_native_context
        }
    }

    pub fn run(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            use emscripten_main_loop::MainLoop;

            struct EngineLoop<'a> {
                game: &'a mut Game,
            }

            impl<'a> MainLoop for EngineLoop<'a> {
                fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
                    self.game.update();
                    self.game.draw();
                    self.game.window.gl_swap_window();
                    emscripten_main_loop::MainLoopEvent::Continue
                }
            }

            let mut app = EngineLoop { game: self };
            emscripten_main_loop::run(app);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Standalone 的遊戲循環
            let mut event_pump = self.game_native_context.sdl_context.event_pump().unwrap();
            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'running,
                        _ => {}
                    }
                }

                self.update();
                self.draw();
                self.game_native_context.window.gl_swap_window();
            }
        }
    }

    fn update(&mut self) {
        if let Some(scene) = &mut self.current_scene {
            scene.update();
        }
    }

    fn draw(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if let Some(scene) = &self.current_scene {
            scene.draw();
        }
    }

    pub fn load_scene(&mut self, scene: Scene) {
        self.current_scene = Some(scene);
    }
}