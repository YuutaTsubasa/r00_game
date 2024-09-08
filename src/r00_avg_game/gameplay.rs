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
    Selection,
    SelectionWait,
    EndCurrent,
    LoadNext,
}

pub struct GamePlayScene {
    current_index: isize,
    next_index: usize,
    avg_items: Vec<AvgItem>,
    status: Status,
    background_plane: Option<Plane>,
    center_character_plane: Option<Plane>,
    character_name_plane: Option<Text>,
    content_plane: Option<Text>,
    selections_texts: Option<Vec<Text>>,

    // static
    frame_plane: Option<Plane>,
    no_name_frame_plane: Option<Plane>,
    selection_background_plane: Option<Plane>,

    // debug
    debug_information_plane: Option<Text>,
}

impl GamePlayScene {
    pub(crate) fn new(avg_items: Vec<AvgItem>) -> Self {
        Self {
            current_index: -1,
            next_index: 0,
            avg_items,
            status: Status::LoadNext,
            background_plane: None,
            center_character_plane: None,
            character_name_plane: None,
            content_plane: None,
            selection_background_plane: None,
            selections_texts: None,
            frame_plane: None,
            no_name_frame_plane: None,
            debug_information_plane: None,
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
const CONFIRM_SOUND_FILE_PATH: &str = "resources/musics/confirm.mp3";


impl Scene for GamePlayScene {
    fn update(&mut self, game: &mut Game, delta_time: f32, hit_position: Option<(i32, i32)>) {
        let empty_string = EMPTY_STRING.to_string();

        #[cfg(debug_assertions)] {
            self.debug_information_plane = Some(game.drawable_generator.generate_text(
                (0.0, 0.0),
                -0.3,
                &format!("{}, {}",
                         delta_time,
                         hit_position
                             .map_or_else(|| EMPTY_STRING.to_string(), |hit_position| format!("({}, {})", hit_position.0, hit_position.1))),
                1.0,
                (1.0, 1.0, 0.0, 1.0),
                FONT_PATH,
                24,
                VERTEX_SHADER,
                FRAGMENT_SHADER
            ));
        }

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

        if self.selection_background_plane.is_none() {
            self.selection_background_plane = Some(game.drawable_generator.generate_plane_from_image(
                (0.0, 0.0, 1920.0, 1080.0),
                -0.4,
                (0.0, 0.0, 0.0, 0.0),
                None,
                VERTEX_SHADER,
                FRAGMENT_SHADER
            ))
        }

        let status = self.status;
        match status {
            Status::EnterCurrent => {
                let fade_speed_per_second = if hit_position.is_some() { IMMEDIATELY_FADE_SPEED } else { FADE_SPEED_PER_SECOND };
                let input_speed_per_second = if hit_position.is_some() { IMMEDIATELY_INPUT_SPEED } else { INPUT_SPEED_PER_SECOND };

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
                if hit_position.is_some() {
                    let avg_item = &self.avg_items[self.current_index as usize];
                    self.status = if avg_item.selection_items.is_some() { Status::Selection } else { Status::EndCurrent };
                    game.audio_manager.play_sound_one_shot(CONFIRM_SOUND_FILE_PATH);
                }
            },
            Status::Selection => {
                if let Some(selection_background_plane) = &mut self.selection_background_plane {
                    selection_background_plane.set_alpha(0.75);
                }

                if let Some(selection_items) = &self.avg_items[self.current_index as usize].selection_items {
                    let selection_items_len = selection_items.len();
                    let selection_height = 1080.0 / selection_items_len as f32;
                    let first_bottom = selection_height / 2.0 - 30.0;
                    self.selections_texts = Some(selection_items
                        .iter().enumerate().map(|(index, selection_item)| {
                            game.drawable_generator.generate_text(
                                (480.0, first_bottom + selection_height * (selection_items_len - index - 1) as f32),
                                -0.5,
                                &selection_item.content,
                                1.0,
                                (1.0, 1.0, 1.0, 1.0),
                                FONT_PATH,
                                60,
                                VERTEX_SHADER,
                                FRAGMENT_SHADER)
                    }).collect::<Vec<_>>());
                }
                self.status = Status::SelectionWait;
            },
            Status::SelectionWait => {
                if let Some(hit_position) = hit_position {
                    if let Some(selection_texts) = &mut self.selections_texts {
                        let selection_texts_len = selection_texts.len();
                        let avg_item = &self.avg_items[self.current_index as usize];

                        for index in 0..selection_texts_len {
                            let mut selection_text = &mut selection_texts[index];
                            if selection_text.contains((hit_position.0 as f32, hit_position.1 as f32)) {
                                selection_text.set_color((1.0, 1.0, 0.0, 1.0));
                                self.next_index = avg_item.selection_items.as_ref().unwrap()[index].next_index as usize;
                                self.status = Status::EndCurrent;
                                game.audio_manager.play_sound_one_shot(CONFIRM_SOUND_FILE_PATH);
                            }
                        }
                    }
                }
            },
            Status::EndCurrent => {
                let fade_speed_per_second = if hit_position.is_some() { IMMEDIATELY_FADE_SPEED } else { FADE_SPEED_PER_SECOND };
                let avg_item = &self.avg_items[self.current_index as usize];
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

                if let Some(selection_background_plane) = &mut self.selection_background_plane {
                    selection_background_plane.set_alpha(
                        (selection_background_plane.drawable.material.color[3] - delta_time * fade_speed_per_second).max(0.0));
                }

                if let Some(selection_texts) = &mut self.selections_texts {
                    let selection_texts_len = selection_texts.len();
                    for index in 0..selection_texts_len {
                        let mut selection_text = &mut selection_texts[index];
                        selection_text.set_alpha((selection_text.drawable.material.color[3] - delta_time * fade_speed_per_second).max(0.0));
                    }
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
                    if self.current_index >= 0 { &self.avg_items.get(self.current_index as usize) }
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

                self.selections_texts = None;

                self.status = Status::EnterCurrent;
                self.current_index = self.next_index as isize;
                self.next_index = avg_item.next_index.unwrap_or_else(|| (self.next_index + 1) as u32) as usize;
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

        if self.status == Status::SelectionWait || self.status == Status::EndCurrent {
            if let Some(selection_background_plane) = &self.selection_background_plane {
                selection_background_plane.draw(game.current_projection_matrix);
            }

            if let Some(selection_texts) = &self.selections_texts {
                for selection_text in selection_texts {
                    selection_text.draw(game.current_projection_matrix);
                }
            }
        }

        if let Some(delta_time_text) = &self.debug_information_plane {
            delta_time_text.draw(game.current_projection_matrix);
        }
    }
}