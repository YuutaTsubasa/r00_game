mod engine;
mod r00_avg_game;

use std::cell::RefCell;
use std::rc::Rc;
use engine::game::Game;
use engine::scene::Scene;
use crate::r00_avg_game::data::avg_item::AvgItem;
use crate::r00_avg_game::gameplay::GamePlayScene;

fn main() {
    #[cfg(target_arch = "wasm32")]
    let is_web = true;
    #[cfg(not(target_arch = "wasm32"))]
    let is_web = false;

    let mut game = Game::new("AVG Game", 960, 540, is_web);
    let mut scene = GamePlayScene::new(
        vec![
            AvgItem {
                background_music: Some("./resources/musics/background.mp3".to_string()),
                background_image_path: Some("./resources/images/background001.png".to_string()),
                center_character_image_path: Some("./resources/images/characters/Yuuta1/sad.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("花了一天將 BUG 修正後......".to_string()),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/normal.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("總算能用 Rust 做一個簡單的 AVG 場景了！".to_string()),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: None,
                character_name: Some("悠太翼".to_string()),
                content: Some("而且還可以建置成 Standalone 和 Web 的 Build！".to_string()),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: None,
                character_name: Some("悠太翼".to_string()),
                content: Some("漸層動畫和打字動畫都補上去了！".to_string()),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/happy.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("好讚！".to_string()),
            }
        ]
    );
    game.load_scene(Rc::new(RefCell::new(scene)));
    game.run();
}