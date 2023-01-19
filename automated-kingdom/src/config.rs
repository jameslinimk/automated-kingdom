use std::fs;

use derive_new::new;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::ternary;

#[cfg(not(target_arch = "wasm32"))]
fn write_to_path<T: AsRef<str>>(path: T, data: T) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path.as_ref())?;
    file.write_all(data.as_ref().as_bytes())?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn write_to_path<T: AsRef<str>>(key: T, data: T) -> Result<(), Box<dyn std::error::Error>> {
    use std::io;

    let window = web_sys::window().unwrap();
    let local_storage = window
        .local_storage()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get local storage"))?
        .unwrap();

    local_storage
        .set_item(key.as_ref(), data.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to write to local storage"))?;

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn get_path<T: AsRef<str>>(path: T) -> Option<String> {
    use std::path::Path;

    let path = Path::new(path.as_ref());
    if !path.exists() {
        return None;
    }
    let contents = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return None,
    };
    Some(contents)
}

#[cfg(target_arch = "wasm32")]
fn get_path<T: AsRef<str>>(key: T) -> Option<String> {
    use std::io;

    macro_rules! unwrap_or_none {
        ($expr:expr) => {
            match $expr {
                Some(val) => val,
                None => None,
            }
        };
    }

    let window = web_sys::window().unwrap();
    let local_storage = unwrap_or_none!(window
        .local_storage()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get local storage"))
        .ok())?;
    let contents = unwrap_or_none!(local_storage
        .get_item(key.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to read local storage"))
        .ok())?;
    Some(contents)
}

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "./automated-kingdom/config.ron";
#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "./config.ron";

#[derive(Debug, Serialize, Deserialize, Clone, Copy, new)]
pub struct Config {
    /// Desired window height
    #[new(value = "1280")]
    pub window_height: i32,

    /// Desired window width
    #[new(value = "1280")]
    pub window_width: i32,

    /// Desired fps limit
    #[new(value = "60")]
    pub fps_limit: u16,
}
impl Config {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let str = to_string_pretty(self, PrettyConfig::new().struct_names(true))?;
        write_to_path(
            CONFIG_PATH,
            &(str + ternary!(cfg!(not(target_os = "windows")), "\n", "\r\n")),
        )?;

        Ok(())
    }

    pub fn load() -> Config {
        let str = get_path(CONFIG_PATH);
        if let Some(str) = str {
            if let Ok(config) = ron::from_str(&str) {
                return config;
            }
        }

        let cfg = Config::new();
        cfg.save().expect("Failed to save config");
        cfg
    }
}

static mut CONFIG: Option<Config> = None;
/// Returns the global [Config] object as a mutable reference
pub fn config() -> &'static mut Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::load());
        }
        CONFIG.as_mut().unwrap()
    }
}
