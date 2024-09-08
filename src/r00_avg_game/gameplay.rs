use std::cmp::{max, PartialEq};
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
    LoadNext,
}

pub struct GamePlayScene {
    next_index: usize,
    avg_items: Vec<AvgItem>,
    status: Status,
    background_plane: Option<Plane>,
    center_character_plane: Option<Plane>,
    character_name_plane: Option<Text>,
    content_plane: Option<Text>,

    // static
    frame_plane: Option<Plane>,
    no_name_frame_plane: Option<Plane>,

    // debug
    delta_time: Option<Text>,
}

impl GamePlayScene {
    pub(crate) fn new(avg_items: Vec<AvgItem>) -> Self {
        Self {
            next_index: 0,
            avg_items,
            status: Status::LoadNext,
            background_plane: None,
            center_character_plane: None,
            character_name_plane: None,
            content_plane: None,
            frame_plane: None,
            no_name_frame_plane: None,
            delta_time: None,
        }
    }
}

const VERTEX_SHADER: &str = include_str!("../shaders/vertex_shader.glsl");
const FRAGMENT_SHADER: &str = include_str!("../shaders/fragment_shader.glsl");
const FONT_PATH: &str = "./resources/fonts/SourceHanSerifTC-Heavy.otf";
const FADE_SPEED_PER_SECOND: f32 = 2.0;
const IMMEDIATELY_FADE_SPEED : f32 = 10000.0;
const INPUT_SPEED_PER_SECOND: f32 = 10.0;
const IMMEDIATELY_INPUT_SPEED : f32 = 10000.0;
const EMPTY_STRING: &str = "";


impl Scene for GamePlayScene {
    fn update(&mut self, game: &mut Game, delta_time: f32, is_hit: bool) {
        let empty_string = EMPTY_STRING.to_string();
        self.delta_time = Some(game.drawable_generator.generate_text(
            (0.0, 0.0),
            -0.3,
            &delta_time.to_string(),
            1.0,
            (1.0, 1.0, 0.0, 1.0),
            FONT_PATH,
            24,
            VERTEX_SHADER,
            FRAGMENT_SHADER
        ));

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
        match status {
            Status::EnterCurrent => {
                let fade_speed_per_second = if is_hit { IMMEDIATELY_FADE_SPEED } else { FADE_SPEED_PER_SECOND };
                let input_speed_per_second = if is_hit { IMMEDIATELY_INPUT_SPEED } else { INPUT_SPEED_PER_SECOND };

                if let Some(background_plane) = &mut self.background_plane {
                    background_plane.set_alpha(
                        (background_plane.drawable.material.color[3] + delta_time * fade_speed_per_second).min(1.0));
                }

                if let Some(center_character_plane) = &mut self.center_character_plane {
                    center_character_plane.set_alpha(
                        (center_character_plane.drawable.material.color[3] + delta_time * fade_speed_per_second).min(1.0));
                }

                if let Some(character_name_text) = &mut self.character_name_plane {
                    character_name_text.set_alpha(
                        (character_name_text.drawable.material.color[3] + delta_time * fade_speed_per_second).min(1.0));
                }

                if let Some(content_text) = &mut self.content_plane {
                    content_text.set_alpha(1.0);
                    content_text.set_range(
                        &game.sdl2_ttf_context,
                        (content_text.end_range_ratio + delta_time * input_speed_per_second / content_text.get_content_char_indices_count() as f32).min(1.0));
                }

                if self.background_plane.as_ref().map_or(true, |plane| plane.drawable.material.color[3] >= 1.0) &&
                   self.center_character_plane.as_ref().map_or(true, |plane| plane.drawable.material.color[3] >= 1.0) &&
                   self.character_name_plane.as_ref().map_or(true, |plane| plane.drawable.material.color[3] >= 1.0) &&
                   self.content_plane.as_ref().map_or(true, |plane| plane.end_range_ratio >= 1.0) {
                    self.status = Status::Wait;
                }
            },
            Status::Wait => {
                if is_hit {
                    self.status = Status::EndCurrent;
                    game.audio_manager.play_sound_one_shot("resources/musics/maou_se_system47.mp3");
                }
            },
            Status::EndCurrent => {
                let fade_speed_per_second = if is_hit { IMMEDIATELY_FADE_SPEED } else { FADE_SPEED_PER_SECOND };
                let avg_item = &self.avg_items[self.next_index - 1];
                let next_avg_item = self.avg_items.get(self.next_index);

                let is_change_background = next_avg_item
                    .and_then(|avg_item| avg_item.background_image_path.as_ref())
                    .is_some();
                if is_change_background {
                    if let Some(background_plane) = &mut self.background_plane {
                        background_plane.set_alpha(
                            (background_plane.drawable.material.color[3] - delta_time * fade_speed_per_second).max(0.0));
                    }
                }

                let is_change_left_character = next_avg_item
                    .and_then(|avg_item| avg_item.center_character_image_path.as_ref())
                    .is_some();
                if is_change_left_character {
                    if let Some(center_character_plane) = &mut self.center_character_plane {
                        center_character_plane.set_alpha(
                            (center_character_plane.drawable.material.color[3] - delta_time * fade_speed_per_second).max(0.0));
                    }
                }

                let current_character_name = avg_item
                    .character_name
                    .as_ref()
                    .unwrap_or(&empty_string)
                    .as_str();
                let next_character_name = next_avg_item
                    .and_then(|avg_item| avg_item.character_name.as_ref())
                    .unwrap_or(&empty_string)
                    .as_str();
                let is_change_character_name = current_character_name != next_character_name;
                if is_change_character_name {
                    if let Some(character_name_plane) = &mut self.character_name_plane {
                        character_name_plane.set_alpha(
                            (character_name_plane.drawable.material.color[3] - delta_time * fade_speed_per_second).max(0.0));
                    }
                }

                if let Some(content_plane) = &mut self.content_plane {
                    content_plane.set_alpha(
                        (content_plane.drawable.material.color[3] - delta_time * fade_speed_per_second).max(0.0));
                }

                if self.background_plane.as_ref().map_or(true, |plane| plane.drawable.material.color[3] <= 0.0 || !is_change_background) &&
                    self.center_character_plane.as_ref().map_or(true, |plane| plane.drawable.material.color[3] <= 0.0 || !is_change_left_character) &&
                    self.character_name_plane.as_ref().map_or(true, |plane| plane.drawable.material.color[3] <= 0.0 || !is_change_character_name) &&
                    self.content_plane.as_ref().map_or(true, |plane| plane.drawable.material.color[3] <= 0.0) {
                    self.status = Status::LoadNext;
                }
            },
            Status::LoadNext => {
                if self.next_index >= self.avg_items.len() {
                    return;
                }
                let previous_avg_item =
                    if self.next_index > 0 { &self.avg_items.get(self.next_index - 1) }
                    else { &None };
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
                            (1.0, 1.0, 1.0, 0.0),
                            Some(background_image_path),
                            VERTEX_SHADER,
                            FRAGMENT_SHADER
                        ))
                    };
                }

                if let Some(center_character_image_path) = &avg_item.center_character_image_path {
                    self.center_character_plane = match center_character_image_path.as_str() {
                        NO_IMAGE => None,
                        _ => Some(game.drawable_generator.generate_plane_from_image(
                            (0.0, 0.0, 1920.0, 1080.0),
                            -0.1,
                            (1.0, 1.0, 1.0, 0.0),
                            Some(center_character_image_path),
                            VERTEX_SHADER,
                            FRAGMENT_SHADER
                        ))
                    };
                }

                let previous_character_name = previous_avg_item
                    .and_then(|avg_item| avg_item.character_name.as_ref())
                    .unwrap_or(&empty_string)
                    .as_str();
                let current_character_name = avg_item
                    .character_name
                    .as_ref()
                    .unwrap_or(&empty_string)
                    .as_str();
                let is_change_character_name = previous_character_name != current_character_name;
                self.character_name_plane = avg_item.character_name
                    .as_ref()
                    .map(|character_name| {
                        game.drawable_generator.generate_text(
                            (16.0, 385.0),
                            -0.3,
                            character_name,
                            1.0,
                            (0.5, 0.7, 1.0, if is_change_character_name { 0.0 } else { 1.0 }),
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
                            0.0,
                            (1.0, 1.0, 1.0, 0.0),
                            FONT_PATH,
                            60,
                            VERTEX_SHADER,
                            FRAGMENT_SHADER
                        )
                    });

                self.status = Status::EnterCurrent;
                self.next_index = self.next_index + 1;
            }
        }

    }

    fn draw(&self, game: &mut Game) {
        if let Some(background_plane) = &self.background_plane {
            background_plane.draw(game.current_projection_matrix);
        }

        if let Some(left_character_plane) = &self.center_character_plane {
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

        if let Some(delta_time_text) = &self.delta_time {
            delta_time_text.draw(game.current_projection_matrix);
        }
    }
}