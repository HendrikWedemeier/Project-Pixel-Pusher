use minifb::{Window, WindowOptions};
use std::sync::{OnceLock, Mutex, MutexGuard};

enum WindowType{
    windowed,
    borderless,
    fullscreen
}

pub struct WindowBuffer{
    buffer: Vec<u32>
}

impl WindowBuffer{
    fn new(width: usize, height: usize) -> Self{
        Self { buffer: vec![0; width * height] }
    }
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
                width: WindowManager::get_window_width(),
                height: WindowManager::get_window_height(),
                _type: WindowManager::get_window_type()

            }))
            .lock()
            .unwrap()
    }

    fn new(width: usize, height: usize, _type: WindowType) -> Self{
        Self{width, height, _type}
    }

    pub fn window_superloop(){

        let mut buffer = 
        WindowBuffer::new(WindowManager::get_window_width(), WindowManager::get_window_height())
        .buffer;
        let mut window_instance = WindowManager::get_instance();

        let mut window = Window::new(
        "Pixel Pusher",
        window_instance.width,
        window_instance.height,
        WindowOptions::default(),
    ).unwrap();

    WindowManager::update_pixel(&mut buffer, 80, 30, 0x000000FF);
    while window.is_open() {
        window.update_with_buffer(&buffer, window_instance.width, window_instance.height).unwrap();
    }
    }

    fn get_window_height() -> usize{
        return 600;
    }

    fn get_window_width() -> usize{
        return 800;
    }

    fn get_window_type() -> WindowType{
        return WindowType::windowed;
    }

    fn set_buffer() -> Vec<u32>{
        let buffer: Vec<u32> = vec![0; WindowManager::get_window_width() * WindowManager::get_window_height()];
        return buffer;
    }

    fn update_pixel(buffer: &mut Vec<u32>, x: usize, y: usize, colour: u32){
    buffer[y * WindowManager::get_window_width() + x] = colour;
}
}