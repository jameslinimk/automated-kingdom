use derive_new::new;
use macroquad::prelude::{vec2, Rect, Texture2D, WHITE};
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::time::get_time;

#[derive(new, Clone, Copy)]
pub struct SpriteSheet {
    /// Texture containing all frames of the sprite sheet
    pub base_texture: Texture2D,
    /// Width of a single frame in the sprite sheet
    pub width: u16,
    /// Duration of a single frame in seconds
    pub frame_duration: f32,

    #[new(value = "f64::MIN")]
    last_frame: f64,

    #[new(value = "0")]
    current_frame: u16,
}
impl SpriteSheet {
    pub fn draw(&self, x: f32, y: f32, width: f32) {
        let w = self.base_texture.width() / self.width as f32;
        let h = self.base_texture.height();

        let rect = Rect::new(self.current_frame as f32 * w, 0.0, w, h);
        draw_texture_ex(
            self.base_texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(rect),
                dest_size: Some(vec2(width, h * (width / w))),
                ..DrawTextureParams::default()
            },
        );
    }

    pub fn update(&mut self) {
        if get_time() > self.last_frame + self.frame_duration as f64 {
            self.current_frame += 1;
            if self.current_frame >= self.width {
                self.current_frame = 0;
            }
            self.last_frame = get_time();
        }
    }

    pub fn pause(&mut self) {
        self.last_frame = f64::MAX;
    }

    pub fn resume(&mut self) {
        if self.last_frame == f64::MAX {
            self.last_frame = get_time();
        }
    }
}
