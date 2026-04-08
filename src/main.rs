use minifb::{Window, WindowOptions};
use crate::{vector_math::vector2d::Vector2d, window::window_manager::WindowManager};
mod vector_math;
mod window;
mod renderer;
fn main() {
    WindowManager::window_superloop();
}


fn calc_vector(start: Vector2d, end: Vector2d){
    
}