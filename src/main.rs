use minifb::{Window, WindowOptions};
use crate::vector_math::vector2d::Vector2d;
mod vector_math;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer : Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Pixel Pusher",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    //roter pixel#
    //buffer[30 * WIDTH + 80] = 0x000000FF;

    update_pixel(&mut buffer, 80, 30, 0x000000FF);
    while window.is_open() {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn update_pixel(buffer: &mut Vec<u32>, x: usize, y: usize, colour: u32){
    buffer[y * WIDTH + x] = colour;
}

fn calc_vector(start: Vector2d, end: Vector2d){
    
}