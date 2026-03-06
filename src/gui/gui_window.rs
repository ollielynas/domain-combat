use macroquad::prelude::*;
use crate::consts::*;


pub enum GuiWindowStyle {
    Basic,
    Debug,
}

pub fn draw_gui_window(title: &str, x: f32, y: f32, h: f32, w: f32, style: GuiWindowStyle) {
    match style {
        GuiWindowStyle::Basic => {
            draw_rectangle(x, y, w, h, GRAY);
            draw_rectangle_lines(x, y, w, h, 3.0, DARKGRAY);
            draw_line(x, y, x + w, y, GUI_WINDOW_TITLE_HEIGHT, BLACK);

            // scale font size down until the text fits within the title bar width
            let padding = 15.0;
            let max_width = w - padding;
            let mut font_size = GUI_WINDOW_TITLE_HEIGHT as u16;
            loop {
                let text_size = measure_text(title, None, font_size, 1.0);
                if text_size.width <= max_width || font_size <= 1 {
                    let text_y = y + ( text_size.height) / 2.0 - 5.0;
                    draw_text(title, x + padding / 2.0, text_y, font_size as f32, WHITE);
                    break;
                }
                font_size -= 1;
            }
        }
        GuiWindowStyle::Debug => {
            draw_line(x, y, x, y + h, 5.0, BLACK);
        }
    }
}
