
pub mod input_source;
pub mod game;
pub mod scenes;

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(WHITE);


        next_frame().await;
        
    }
}