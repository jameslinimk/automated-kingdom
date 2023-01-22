#[cfg(windows)]
use winres::WindowsResource;

fn main() {
    #[cfg(windows)]
    #[allow(unused_must_use)]
    {
        WindowsResource::new().set_icon("icon.ico").compile();
    }
}
