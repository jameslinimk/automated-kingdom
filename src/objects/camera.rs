//! Contains the camera object, used to manually control the camera or to pan to a target

use std::f32::consts::FRAC_1_SQRT_2;

use derive_new::new;
use macroquad::prelude::rand::gen_range;
use macroquad::prelude::{
    get_frame_time, get_time, is_key_down, screen_height, screen_width, set_camera, vec2, Camera2D,
    KeyCode, Vec2,
};

use crate::math::{angle, distance, ease_in_out, project};

/// Info about the a camera shake, sent to the camera to start a shake
#[derive(Debug, Clone, Copy)]
pub struct ShakeConfig {
    pub duration: f32,
    pub intensity: f32,
}

/// The camera object, used to manually control the camera or to pan to a target
#[derive(new)]
pub struct Camera {
    /// Actual [Camera2D] object sent to Macroquad
    #[new(value = "Camera2D {
        zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
        target: vec2(screen_width() / 2.0, screen_height() / 2.0),
        ..Default::default()
    }")]
    pub camera: Camera2D,

    /// Set's the target of the camera, `None` if no target is active
    /// - If active, the camera will pan to the target and will lock manual movement
    #[new(value = "None")]
    pub target: Option<Vec2>,

    /// The current camera shake config, `None` if no shake is active
    #[new(value = "None")]
    pub shake: Option<ShakeConfig>,

    /// The current offset of the camera shake
    #[new(value = "vec2(0.0, 0.0)")]
    pub shake_offset: Vec2,

    /// The time the current shake started
    #[new(value = "0.0")]
    shake_start: f64,

    /// The speed of the camera when in manual mode
    #[new(value = "500.0")]
    pub speed: f32,

    /// Bottom right of the camera bounds, top left is always `0,0`
    #[new(value = "Some(vec2(2000.0, 2000.0))")]
    pub bounds: Option<Vec2>,
}
impl Camera {
    /// Updates the camera, should be called every frame
    pub fn update(&mut self) {
        /* --------------------------------- Target --------------------------------- */
        if let Some(target) = self.target {
            let dis = distance(&self.camera.target, &target);
            let max_increase = screen_width().max(screen_height()) / 2.0;

            let ratio = ease_in_out(dis / max_increase);

            let pan_speed = (2000.0 * ratio) * get_frame_time();

            if dis > pan_speed {
                let angle = angle(&self.camera.target, &target);
                self.camera.target = project(&self.camera.target, angle, pan_speed);
            }

        /* ----------------------------- Manual control ----------------------------- */
        } else {
            let mut hspd = 0.0;
            let mut vspd = 0.0;

            if is_key_down(KeyCode::W) {
                vspd -= 1.0;
            }
            if is_key_down(KeyCode::S) {
                vspd += 1.0;
            }
            if is_key_down(KeyCode::A) {
                hspd -= 1.0;
            }
            if is_key_down(KeyCode::D) {
                hspd += 1.0;
            }

            let dia = if hspd != 0.0 && vspd != 0.0 {
                FRAC_1_SQRT_2
            } else {
                1.0
            };

            self.camera.target.x += hspd * get_frame_time() * self.speed * dia;
            self.camera.target.y += vspd * get_frame_time() * self.speed * dia;

            if let Some(bounds) = self.bounds {
                if self.camera.target.x < screen_width() / 2.0 {
                    self.camera.target.x = screen_width() / 2.0;
                }
                if self.camera.target.y < screen_height() / 2.0 {
                    self.camera.target.y = screen_height() / 2.0;
                }

                if self.camera.target.x > bounds.x - screen_width() / 2.0 {
                    self.camera.target.x = bounds.x - screen_width() / 2.0;
                }
                if self.camera.target.y > bounds.y - screen_height() / 2.0 {
                    self.camera.target.y = bounds.y - screen_height() / 2.0;
                }
            }
        }

        /* ---------------------------------- Shake --------------------------------- */
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

    /// Sets the camera shake, will override any current shake
    pub fn set_shake(&mut self, shake: ShakeConfig) {
        self.shake = Option::from(shake);
        self.shake_start = get_time();
    }

    /// Removes the current camera shake
    pub fn remove_shake(&mut self) {
        self.shake = None;
    }

    /// Sets the camera target, will override any current target
    pub fn set_target(&mut self, target: Vec2) {
        self.target = Some(target);
    }

    /// Removes the current camera target
    pub fn remove_target(&mut self) {
        self.target = None;
    }
}
