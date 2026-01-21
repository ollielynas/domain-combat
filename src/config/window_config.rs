use macroquad::prelude::*;
use crate::consts::*;

pub fn get_window_config() -> Conf {
    return Conf {
        window_title: "Public Domain Combat".to_string(),
        window_width: WINDOW_SIZE.0,
        window_height: WINDOW_SIZE.1,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        // platform: None,
        ..Default::default()
    };
}
