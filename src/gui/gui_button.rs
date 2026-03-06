use macroquad::prelude::*;
use crate::consts::*;


pub enum GuiButtonStyle {
    Basic,
}
pub struct ButtonState {
    pub down: bool,
    pub pressed: bool,
    pub hovered: bool,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self { down: Default::default(), pressed: Default::default(), hovered: Default::default() }
    }
}

pub fn draw_gui_button(text: &str, x: f32, y: f32, style: GuiButtonStyle) -> ButtonState {
    let mut state = ButtonState::default();
    match style {
        GuiButtonStyle::Basic => {
            let text_size = measure_text(text, None, 20, 1.0);

            draw_rectangle(x - MARGIN / 2.0, y - MARGIN / 2.0, text_size.width + MARGIN / 2.0, text_size.height + MARGIN / 2.0, LIGHTGRAY);
            let mouse_pos = mouse_position();

            draw_text(text, x, y + 10.0, 20.0, BLACK);

            if
                mouse_pos.0 > x - MARGIN / 2.0
                && mouse_pos.0 < x + text_size.width + MARGIN / 2.0
                && mouse_pos.1 > y - MARGIN / 2.0
                && mouse_pos.1 < y + text_size.height + MARGIN / 2.0 {
                    draw_rectangle_lines(x - MARGIN / 2.0, y - MARGIN / 2.0, text_size.width + MARGIN / 2.0, text_size.height + MARGIN / 2.0, 3.0 ,BLUE);
                    state.hovered = true;
                    if is_mouse_button_down(MouseButton::Left) {
                        state.down = true;
                        draw_rectangle_lines(x - MARGIN / 2.0, y - MARGIN / 2.0, text_size.width + MARGIN / 2.0, text_size.height + MARGIN / 2.0, 3.0 ,BLACK);
                    }
                    if is_mouse_button_pressed(MouseButton::Left) {
                        state.pressed = true;
                    }
                }


        }
    };
    return state;

}
