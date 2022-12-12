use derive_new::new;
use macroquad::prelude::rand::gen_range;
use macroquad::prelude::{
    get_frame_time, get_time, is_key_pressed, screen_height, screen_width, set_camera, vec2,
    Camera2D, KeyCode, Vec2,
};

use crate::math::{angle, distance, ease_in_out, project};

#[derive(Debug, Clone, Copy)]
pub struct ShakeConfig {
    pub duration: f32,
    pub intensity: f32,
}

#[derive(new)]
pub struct Camera {
    #[new(value = "Camera2D {
        zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
        target: vec2(screen_width() / 2.0, screen_height() / 2.0),
        ..Default::default()
    }")]
    pub camera: Camera2D,

    #[new(value = "None")]
    pub target: Option<Vec2>,

    #[new(value = "None")]
    pub shake: Option<ShakeConfig>,

    #[new(value = "vec2(0.0, 0.0)")]
    pub shake_offset: Vec2,

    #[new(value = "0.0")]
    pub shake_start: f64,

    #[new(value = "0.0")]
    pub speed: f32,
}
impl Camera {
    pub fn update(&mut self) {
        if let Some(target) = self.target {
            let dis = distance(self.camera.target, target);
            let max_increase = screen_width().max(screen_height()) / 2.0;

            let ratio = ease_in_out(dis / max_increase);

            let pan_speed = (2000.0 * ratio) * get_frame_time();

            if dis > pan_speed {
                let angle = angle(self.camera.target, target);
                self.camera.target = project(self.camera.target, angle, pan_speed);
            }
        } else {
            let mut hspd = 0.0;
            let mut vspd = 0.0;

            if is_key_pressed(KeyCode::W) {
                vspd -= 1.0;
            }
            if is_key_pressed(KeyCode::S) {
                vspd += 1.0;
            }
            if is_key_pressed(KeyCode::A) {
                hspd -= 1.0;
            }
            if is_key_pressed(KeyCode::D) {
                hspd += 1.0;
            }

            self.camera.target.x += hspd * get_frame_time() * self.speed;
            self.camera.target.y += vspd * get_frame_time() * self.speed;
        }

        if let Some(shake) = self.shake {
            if self.shake_start == 0.0 || get_time() > self.shake_start + shake.duration as f64 {
                self.shake = None;
            } else {
                let intense = -shake.intensity * get_frame_time();

                self.shake_offset.x = gen_range(-intense, intense);
                self.shake_offset.y = gen_range(-intense, intense);

                self.camera.target.x += self.shake_offset.x;
                self.camera.target.y += self.shake_offset.y;
            }
        }

        set_camera(&self.camera);
    }

    pub fn set_shake(&mut self, shake: ShakeConfig) {
        self.shake = Option::from(shake);
        self.shake_start = get_time();
    }

    pub fn remove_shake(&mut self) {
        self.shake = None;
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.target = Some(target);
    }

    pub fn remove_target(&mut self) {
        self.target = None;
    }
}
