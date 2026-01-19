pub mod config;
pub mod game;
pub mod input_source;
pub mod scenes;

use macroquad::prelude::*;

use crate::config::window_config::get_window_config;
use crate::scenes::{input_select::InputSelect, scenes::Scene};

#[macroquad::main(get_window_config)]
async fn main() {
    let mut current_scene = Scene::InputSelectScene(InputSelect::default());

    loop {
        clear_background(WHITE);

        current_scene.render();

        if let Some(new_scene) = current_scene.change_scene() {
            current_scene = new_scene;
        }

        next_frame().await;
    }
}
