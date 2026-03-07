pub mod config;
pub mod game;
pub mod input_source;
pub mod scenes;
pub mod gui;
pub mod consts;
pub mod animations;

use std::thread::sleep;
use std::time;

use macroquad::prelude::*;

use crate::config::window_config::get_window_config;
use crate::input_source::http_input_device::init_adb_detection;
use crate::scenes::{input_select::InputSelect, scenes::Scene};

#[macroquad::main(get_window_config)]
async fn main() {

    init_adb_detection();
    let mut current_scene = Scene::InputSelectScene(InputSelect::default());

    loop {
        clear_background(WHITE);

        current_scene.render();

        if let Some(new_scene) = current_scene.change_scene() {
            current_scene = new_scene;
        }

        // let ten_millis = time::Duration::from_millis(100);
        // sleep(ten_millis);

        next_frame().await;
    }
}
