use std::cmp::PartialEq;
use std::string::ToString;
use nalgebra_glm::Mat4;
use sdl2::mouse::SystemCursor::No;
use crate::engine::drawable_component::DrawableComponent;
use crate::engine::drawable_implements::plane::Plane;
use crate::engine::drawable_implements::text::Text;
use crate::engine::game::Game;
use crate::engine::scene::Scene;
use crate::r00_avg_game::data::avg_item::AvgItem;

const NO_IMAGE : &str = "NO_IMAGE";

#[derive(Copy, Clone, PartialEq)]
enum Status {
    EnterCurrent,
    Wait,
    EndCurrent,
}

pub struct GamePlayScene {
    next_index: usize,
    avg_items: Vec<AvgItem>,
    status: Status,
    background_plane: Option<Plane>,
    left_character_plane: Option<Plane>,
    character_name_plane: Option<Text>,
    content_plane: Option<Text>,

    // static
    frame_plane: Option<Plane>,
    no_name_frame_plane: Option<Plane>,
}

impl GamePlayScene {
    pub(crate) fn new(avg_items: Vec<AvgItem>) -> Self {
        Self {
            next_index: 0,
            avg_items,
            status: Status::EndCurrent,
            background_plane: None,
            left_character_plane: None,
            character_name_plane: None,
            content_plane: None,
            frame_plane: None,
            no_name_frame_plane: None,
        }
    }
}

const VERTEX_SHADER: &str = include_str!("../shaders/vertex_shader.glsl");
const FRAGMENT_SHADER: &str = include_str!("../shaders/fragment_shader.glsl");
const FONT_PATH: &str = "./resources/fonts/SourceHanSerifTC-Heavy.otf";

impl Scene for GamePlayScene {
    fn update(&mut self, game: &mut Game, is_hit: bool) {
        if self.frame_plane.is_none() {
            self.frame_plane = Some(game.drawable_generator.generate_plane_from_image(
                (0.0, 0.0, 1920.0, 1080.0),
                -0.2,
                (1.0, 1.0, 1.0, 1.0),
                Some(&"resources/images/frame.png".to_string()),
                VERTEX_SHADER,
                FRAGMENT_SHADER
            ))
        }

        if self.no_name_frame_plane.is_none() {
            self.no_name_frame_plane = Some(game.drawable_generator.generate_plane_from_image(
                (0.0, 0.0, 1920.0, 1080.0),
                -0.2,
                (1.0, 1.0, 1.0, 1.0),
                Some(&"resources/images/frame_no_name.png".to_string()),
                VERTEX_SHADER,
                FRAGMENT_SHADER
            ))
        }

        let status = self.status;
        if status == Status::Wait {
            if is_hit {
                self.status = Status::EndCurrent;
            }
        }

        if status == Status::EndCurrent {
            if self.next_index >= self.avg_items.len() {
                return;
            }

            let avg_item = &self.avg_items[self.next_index];
            if let Some(background_music) = &avg_item.background_music {
                game.audio_manager.load_music(background_music);
                game.audio_manager.play_music();
            }

            if let Some(background_image_path) = &avg_item.background_image_path {
                self.background_plane = match background_image_path.as_str() {
                    NO_IMAGE => None,
                    _ => Some(game.drawable_generator.generate_plane_from_image(
                        (0.0, 0.0, 1920.0, 1080.0),
                        0.0,
                        (1.0, 1.0, 1.0, 1.0),
                        Some(background_image_path),
                        VERTEX_SHADER,
                        FRAGMENT_SHADER
                    ))
                };
            }

            if let Some(left_character_image_path) = &avg_item.left_character_image_path {
                self.left_character_plane = match left_character_image_path.as_str() {
                    NO_IMAGE => None,
                    _ => Some(game.drawable_generator.generate_plane_from_image(
                        (0.0, 0.0, 1920.0, 1080.0),
                        -0.1,
                        (1.0, 1.0, 1.0, 1.0),
                        Some(left_character_image_path),
                        VERTEX_SHADER,
                        FRAGMENT_SHADER
                    ))
                };
            }

            self.character_name_plane = avg_item.character_name
                .as_ref()
                .map(|character_name| {
                   game.drawable_generator.generate_text(
                       (16.0, 385.0),
                       -0.3,
                       character_name,
                       (0.5, 0.7, 1.0, 1.0),
                       FONT_PATH,
                       120,
                       VERTEX_SHADER,
                       FRAGMENT_SHADER
                   )
                });

            self.content_plane = avg_item.content
                .as_ref()
                .map(|content| {
                    game.drawable_generator.generate_text(
                        (16.0, 260.0),
                        -0.3,
                        content,
                        (1.0, 1.0, 1.0, 1.0),
                        FONT_PATH,
                        60,
                        VERTEX_SHADER,
                        FRAGMENT_SHADER
                    )
                });

            self.status = Status::Wait;
            self.next_index = self.next_index + 1;
        }
    }

    fn draw(&self, game: &mut Game) {
        if let Some(background_plane) = &self.background_plane {
            background_plane.draw(game.current_projection_matrix);
        }

        if let Some(left_character_plane) = &self.left_character_plane {
            left_character_plane.draw(game.current_projection_matrix);
        }

        if self.content_plane.is_some() {
            if self.character_name_plane.is_some() {
                if let Some(frame_plane) = &self.frame_plane {
                    frame_plane.draw(game.current_projection_matrix);
                }
            }
            else {
                if let Some(no_name_frame_plane) = &self.no_name_frame_plane {
                    no_name_frame_plane.draw(game.current_projection_matrix);
                }
            }
        }

        if let Some(character_name_plane) = &self.character_name_plane {
            character_name_plane.draw(game.current_projection_matrix);
        }

        if let Some(content_plane) = &self.content_plane {
            content_plane.draw(game.current_projection_matrix);
        }
    }
}