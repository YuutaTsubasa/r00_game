mod engine;
mod r00_avg_game;

use std::cell::RefCell;
use std::rc::Rc;
use engine::game::Game;
use engine::scene::Scene;
use crate::r00_avg_game::data::avg_item::{AvgItem, SelectionItem};
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
                center_character_image_path: Some("./resources/images/characters/Yuuta1/normal.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("今天又是美好的一天呢！".to_string()),
                selection_items: Some(vec![
                    SelectionItem {
                        content: "怎麼了？看起來心情不錯！".to_string(),
                        next_index: 1
                    },
                    SelectionItem {
                        content: "還是跟昨天一樣普通吧。".to_string(),
                        next_index: 2
                    }
                ]),
                next_index: None,
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/happy.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("你居然看得出來！對，我今天超開心的！".to_string()),
                selection_items: None,
                next_index: Some(3),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/sad.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("唉，其實今天也沒什麼特別的……".to_string()),
                selection_items: None,
                next_index: Some(3),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/normal.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("總之，今天的任務就這麼完成了！".to_string()),
                selection_items: Some(vec![
                    SelectionItem {
                        content: "恭喜你！".to_string(),
                        next_index: 4
                    },
                    SelectionItem {
                        content: "不就是日常嘛。".to_string(),
                        next_index: 5
                    }
                ]),
                next_index: None,
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/happy.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("謝謝！你真的是個好夥伴！".to_string()),
                selection_items: None,
                next_index: Some(6),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/sad.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("唉，或許你說得對。".to_string()),
                selection_items: None,
                next_index: None,
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/normal.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("對了，你喜歡烤肉嗎？".to_string()),
                selection_items: Some(vec![
                    SelectionItem {
                        content: "當然！烤肉超好吃！".to_string(),
                        next_index: 7
                    },
                    SelectionItem {
                        content: "還好，我更喜歡別的料理。".to_string(),
                        next_index: 8
                    }
                ]),
                next_index: None,
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/happy.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("我就知道你跟我一樣！烤肉最棒了，特別是和朋友們一起烤！".to_string()),
                selection_items: None,
                next_index: Some(9),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/normal.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("真的嗎？烤肉可是超多人喜歡的呢。不過每個人喜好不同嘛～".to_string()),
                selection_items: None,
                next_index: Some(9),
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/normal.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("你最喜歡的烤肉食材是什麼呢？".to_string()),
                selection_items: Some(vec![
                    SelectionItem {
                        content: "牛肉".to_string(),
                        next_index: 10
                    },
                    SelectionItem {
                        content: "豬肉".to_string(),
                        next_index: 10
                    },
                    SelectionItem {
                        content: "海鮮".to_string(),
                        next_index: 10
                    },
                    SelectionItem {
                        content: "蔬菜".to_string(),
                        next_index: 10
                    }
                ]),
                next_index: None,
            },
            AvgItem {
                background_music: None,
                background_image_path: None,
                center_character_image_path: Some("./resources/images/characters/Yuuta1/happy.png".to_string()),
                character_name: Some("悠太翼".to_string()),
                content: Some("嗯嗯，那是我的最愛之一呢！下次一起烤吧～".to_string()),
                selection_items: None,
                next_index: None,
            }
        ]
    );
    game.load_scene(Rc::new(RefCell::new(scene)));
    game.run();
}