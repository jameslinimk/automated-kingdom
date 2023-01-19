use std::fs;

use derive_new::new;
use serde::{Deserialize, Serialize};

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
fn get_path<T: AsRef<str>>(path: T) -> Option<String> {
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
        .get_item(path.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to read local storage"))
        .ok())?;
    Some(contents)
}

const CONFIG_PATH: &str = "./config.ron";

#[derive(Serialize, Deserialize, Clone, Copy, new)]
pub struct Config {
    /// Desired window height
    #[new(value = "1280")]
    pub window_height: u16,

    /// Desired window width
    #[new(value = "1280")]
    pub window_width: u16,

    /// Desired fps limit
    #[new(value = "60")]
    pub fps_limit: u16,
}
impl Config {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let str = ron::to_string(self)?;
        write_to_path(CONFIG_PATH, &str)?;

        Ok(())
    }
}
