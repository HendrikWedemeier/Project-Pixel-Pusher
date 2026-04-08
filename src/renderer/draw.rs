pub mod Draw{
    use crate::{vector_math::vector2d::Vector2d, window::window_manager::WindowManager};

    pub fn draw_line(start_point: &Vector2d, end_point: &Vector2d, pixel_buffer: &mut Vec<u32>, colour: &mut u32){
        let v = Vector2d::sub(end_point, start_point);
        let length = Vector2d::get_length(start_point, end_point);

        let mut point = Vector2d::new(0.0, 0.0);
        for i in 1..=length as i32{
            let lambda = i as f64 / length;
            point = Vector2d::add(start_point, &Vector2d::new(v.x * lambda, v.y * lambda));
            WindowManager::update_pixel(pixel_buffer, point.x as usize, point.y as usize, colour);
        }
    }
}