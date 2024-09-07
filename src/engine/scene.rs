use crate::engine::game::Game;

pub trait Scene {
    fn update(&mut self, game: &mut Game, is_hit: bool);
    fn draw(&self, game: &mut Game);
}
