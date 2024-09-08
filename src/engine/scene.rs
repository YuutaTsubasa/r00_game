use crate::engine::game::Game;

pub trait Scene {
    fn update(&mut self, game: &mut Game, delta_time: f32, is_hit: bool);
    fn draw(&self, game: &mut Game);
}
