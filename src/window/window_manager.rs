use minifb::{Window, WindowOptions};
use std::sync::{OnceLock, Mutex, MutexGuard};

enum WindowType{
    windowed,
    borderless,
    fullscreen
}

pub struct WindowManager{
    width: usize,
    height: usize,
    _type: WindowType
}

static INSTANCE: OnceLock<Mutex<WindowManager>> = OnceLock::new();

impl WindowManager{

    pub fn get_instance() -> MutexGuard<'static, WindowManager> {
        INSTANCE
            .get_or_init(|| Mutex::new(WindowManager {
                width: get_window_width(),
                height: get_window_height(),
                _type: get_window_type(),
            }))
            .lock()
            .unwrap()
    }

    fn new(width: usize, height: usize, _type : WindowType) -> Self{
        Self{width, height, _type}
    }

    fn window_superloop(){

    }

    fn get_window_height() -> i32{

    }

    fn get_window_width() -> i32{

    }

    fn get_window_type() -> WindowType{

    }
}