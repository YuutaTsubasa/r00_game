use crate::engine::game::Game;

pub trait Scene {
    fn update(&mut self, game: &mut Game, delta_time: f32, hit_position: Option<(i32, i32)>);
    fn draw(&self, game: &mut Game);
}
