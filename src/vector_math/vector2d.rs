pub struct Vector2d {
    x: i32,
    y: i32
}

impl Vector2d{
    fn new(x: i32, y: i32) -> Self{
        Self { x, y }
    }

    fn mult(vector:&mut Vector2d, lamda: f64) -> Vector2d{
        return Vector2d::new
        (
            (vector.x as f64 * lamda) as i32,
            (vector.y as f64 * lamda) as i32
        );
    }
}