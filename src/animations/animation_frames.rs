use macroquad::{math::Rect, texture::{Image, Texture2D}};
use macroquad::prelude::*;
use crate::{animations::animation_manager::Ani, consts::DEBUG_TEXT_SIZE};



pub struct AnimationFrames {
    pub do_loop: bool,
    pub randomise: bool,
    pub anamaiton_state: Ani,
    animation_duration: f32,
    spritesheet: Image,
    texture: Option<Texture2D>,
    frames: Vec<AniFrame>,
    frame_map: Vec<usize>,
}

pub struct AniFrame {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    offset_x: f32,
    offset_y: f32,
    rel_scale_height: f32,
    time_multiplier: u8,
}

pub struct AnimationFramesConstructor {
    frames: Vec<Image>,
    offsets: Vec<(f32,f32)>,
    rel_sclae_height: Vec<f32>,
    /// a multiplier of two would change the time the frame is showen from 1/10th of the whole animation to 2/11, if there were 9 other frames, which all had a multiplier of 1
    frame_time_multiplier: Vec<u8>,

    flip_x: Vec<bool>

}

impl AnimationFramesConstructor {
    pub fn new() -> Self {
        return AnimationFramesConstructor { flip_x:vec![], frame_time_multiplier: vec![], frames: vec![], offsets: vec![], rel_sclae_height: vec![] }
    }

    pub fn add_frame(&mut self, bytes: &[u8]) {
        let image = Image::from_file_with_format(bytes, None);
        if let Ok(img) = image {
            self.frames.push(img);
            self.offsets.push((0.0,0.0));
            self.rel_sclae_height.push(1.0);
            self.flip_x.push(false);
            self.frame_time_multiplier.push(1);
        }
    }

    pub fn change_proportional_frame_time(&mut self, multiplier: u8) {
        if let Some(last_time) =  self.frame_time_multiplier.last_mut() {
            *last_time = multiplier;
        }
    }

    /// the char should face right
    pub fn flip_x(&mut self) {
        if let Some(x) =  self.flip_x.last_mut() {
            *x = ! *x;
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        if self.rel_sclae_height.len() > 0 {
            *(self.rel_sclae_height.last_mut().unwrap()) = scale;
        }
    }
    /// aligns to bottom of hitbox
    pub fn align_bottom(&mut self) {
        if let Some(scale) =  self.rel_sclae_height.last() {
        if let Some(last_offset) =  self.offsets.last_mut() {
        let y_offset = &mut last_offset.1;
        *y_offset = (1.0 - scale) / 2.0;
        }
        }
    }

    /// aligns to top of hitbox
    pub fn align_top(&mut self) {
        if let Some(scale) =  self.rel_sclae_height.last() {
        if let Some(last_offset) =  self.offsets.last_mut() {
        let y_offset = &mut last_offset.1;
        *y_offset = - (1.0 - scale) / 2.0;
        }
        }
    }
    /// alligns to left side of hitbox
    /// assume char is alwase facing right and that the char has a height of one, if the hitbox is twice as tall as it is wide the aspect ration should be 0.5
    pub fn align_left(&mut self, aspect_ratio: f32) {
        if let Some(scale) =  self.rel_sclae_height.last() {
        if let Some(last_offset) =  self.offsets.last_mut() {
        let x_offset = &mut last_offset.0;
        *x_offset = (0.5 * scale) - aspect_ratio / 2.0;
        }
        }
    }
    /// alighs to right side of hitbox
    /// assume char is alwase facing right and that the char has a height of one, if the hitbox is twice as tall as it is wide the aspect ration should be 0.5
    pub fn align_right(&mut self, aspect_ratio: f32) {
        if let Some(scale) =  self.rel_sclae_height.last() {
        if let Some(last_offset) =  self.offsets.last_mut() {
        let x_offset = &mut last_offset.0;
        *x_offset = (0.5 * scale) - aspect_ratio / 2.0;
        }
        }
    }

    pub fn build(&self, do_loop: bool, randomise: bool, animation_state: Ani, animation_duration: f32) -> AnimationFrames {
        // Find the total width and max height for the spritesheet
        let total_width: i32 = self.frames.iter().map(|f| f.width() as i32).sum();
        let max_height: i32 = self.frames.iter().map(|f| f.height() as i32).max().unwrap_or(0);

        // Create a blank RGBA image for the spritesheet
        let mut sheet_bytes = vec![0u8; (total_width * max_height * 4) as usize];

        let mut ani_frames: Vec<AniFrame> = Vec::new();
        let mut cursor_x: i32 = 0;

        let mut frame_map = vec![];
        for (l, dr) in self.frame_time_multiplier.iter().enumerate() {
            for i in 0_u8..*dr {
                frame_map.push(l);
            }
        }



        for i in 0..self.frames.len() {
            let frame = &self.frames[i];
            let fw = frame.width() as i32;
            let fh = frame.height() as i32;

            // Copy frame pixels into the sheet row by row
            for row in 0..fh {
                for col in 0..fw {
                    let src_idx = ((row * fw + col) * 4) as usize;
                    let dst_idx = ((row * total_width + cursor_x + col) * 4) as usize;
                    let pixel = frame.get_pixel(if self.flip_x[i] {fw-col - 1} else {col} as u32, row as u32);
                    sheet_bytes[dst_idx    ] = (pixel.r * 255.0) as u8;
                    sheet_bytes[dst_idx + 1] = (pixel.g * 255.0) as u8;
                    sheet_bytes[dst_idx + 2] = (pixel.b * 255.0) as u8;
                    sheet_bytes[dst_idx + 3] = (pixel.a * 255.0) as u8;
                }
            }

            ani_frames.push(AniFrame {
                x: cursor_x,
                y: 0,
                width: fw,
                height: fh,
                offset_x: self.offsets[i].0,
                offset_y: self.offsets[i].1,
                rel_scale_height: self.rel_sclae_height[i],
                time_multiplier: self.frame_time_multiplier[i]
            });

            cursor_x += fw;
        }

        let spritesheet = Image::gen_image_color(
            total_width as u16,
            max_height as u16,
            Color::new(0.0, 0.0, 0.0, 0.0),
        );

        // Overwrite the blank image's bytes with our packed data
        let mut spritesheet = spritesheet;
        spritesheet.bytes = sheet_bytes;

        AnimationFrames {
            do_loop,
            randomise,
            anamaiton_state: animation_state,
            spritesheet,
            texture: None,
            animation_duration,
            frames: ani_frames,
            frame_map,
        }
    }

}

impl AnimationFrames {
    pub fn render_frame(&mut self, x: f32, y: f32, time: f32, height: f32, flip_x: bool) {
        if self.texture.is_none() {
            self.texture = Some(Texture2D::from_image(&self.spritesheet));
        }
        let texture = self.texture.as_ref().unwrap();

        let time = if !self.do_loop {
            time.min(self.animation_duration)
        } else {
            time % self.animation_duration
        };

        let frame_index = if self.randomise {
            *fastrand::choice(self.frame_map.iter()).unwrap()
        } else {
            ((time / self.animation_duration * self.frame_map.len() as f32) as usize)
                .clamp(0, self.frame_map.len() - 1)
        };

        let frame = &self.frames[frame_index];

        let draw_height = height * frame.rel_scale_height;
        let draw_width = (frame.width as f32 / frame.height as f32) * draw_height;

        // Center the sprite on (x, y), then apply offsets
        let draw_x = x - draw_width  * 0.5 + frame.offset_x * height;
        let draw_y = y - draw_height * 0.5 + frame.offset_y * height;


        draw_texture_ex(texture, draw_x, draw_y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(draw_width, draw_height)),
            source: Some(Rect::new(frame.x as f32, frame.y as f32, frame.width as f32, frame.height as f32)),
            rotation: 0.0,
            flip_x,
            flip_y: false,
            pivot: Default::default(),
        });
    }
}
