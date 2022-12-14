use ak_server::types_game::{Sprite, Texture};
use derive_new::new;
use macroquad::prelude::{Rect, Texture2D, WHITE};
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::time::get_time;

use crate::texture_map::TextureMap;

#[derive(Debug, Clone, Copy, new)]
pub struct SpriteSheet {
    /// Texture containing all frames of the sprite sheet
    pub texture: Texture,
    /// Duration of a single frame in seconds
    pub frame_duration: f32,

    #[new(value = "f64::MIN")]
    last_frame: f64,
    #[new(value = "0")]
    current_frame: u16,
    #[new(value = "texture.texture()")]
    base_texture: Texture2D,
    #[new(value = "(texture.texture().height() / texture.texture().width()) as u16")]
    frames: u16,
}
impl SpriteSheet {
    /// Creates a new `SpriteSheet` from a given fps
    pub fn new_fps(texture: Texture, fps: f32) -> SpriteSheet {
        SpriteSheet::new(texture, 1.0 / fps)
    }

    /// Will draw the current frame of the sprite sheet at given top-left position
    pub fn draw(&self, x: f32, y: f32) {
        let w = self.base_texture.width() / self.frames as f32;
        let h = self.base_texture.height();

        let rect = Rect::new(self.current_frame as f32 * w, 0.0, w, h);
        draw_texture_ex(
            self.base_texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(rect),
                ..Default::default()
            },
        );
    }

    /// Converts `Self` to a [Sprite]
    pub fn as_server(&self) -> Sprite {
        Sprite::SpriteSheet {
            texture: self.texture,
            frames: self.frames,
            current_frame: self.current_frame,
        }
    }

    /// Updates the current frame of the sprite sheet, call every frame
    pub fn update(&mut self) {
        if get_time() > self.last_frame + self.frame_duration as f64 {
            self.current_frame += 1;
            if self.current_frame >= self.frames {
                self.current_frame = 0;
            }
            self.last_frame = get_time();
        }
    }

    /// Pause the spritesheet
    pub fn pause(&mut self) {
        self.last_frame = f64::MAX;
    }

    /// Check if the spritesheet is paused
    pub fn paused(&self) -> bool {
        self.last_frame == f64::MAX
    }

    /// Resume the spritesheet if paused
    pub fn resume(&mut self) {
        if self.last_frame == f64::MAX {
            self.last_frame = get_time();
        }
    }
}
