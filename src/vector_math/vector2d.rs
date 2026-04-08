pub struct Vector2d {
    pub(crate) x: f64,
    pub(crate) y: f64
}

impl Vector2d{
    pub fn new(x: f64, y: f64) -> Self{
        Self { x, y }
    }

    pub fn add(v_a: &Vector2d, v_b: &Vector2d) -> Vector2d{
        return Vector2d::new
        (
            v_a.x + v_b.x,
            v_a.y + v_b.y
        );
    }

    pub fn sub(v_a: &Vector2d, v_b: &Vector2d) -> Vector2d{
        return Vector2d::new
        (
            v_a.x - v_b.x,
            v_a.y - v_b.y
        );
    }

    pub fn mult(vector: &Vector2d, lamda: f64) -> Vector2d{
        return Vector2d::new
        (
            vector.x * lamda,
            vector.y * lamda
        );
    }

    //TODO: Fast approximate squere root

    pub fn get_length(start_point: &Vector2d, end_point: &Vector2d) -> f64{

        let x = f64::powf(end_point.x - start_point.x, 2.0) + f64::powf(end_point.y - start_point.y, 2.0 );
        return f64::sqrt(x);
    }
}