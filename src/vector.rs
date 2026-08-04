use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn size(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }

    pub fn add(&self, v: Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    pub fn mul(self, n: f32) -> Vector {
        Vector {
            x: self.x * n,
            y: self.y * n,
        }
    }

    pub fn dot(self, v: Vector) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn flip_x(&mut self) {
        self.x = -self.x;
    }
    pub fn flip_y(&mut self) {
        self.y = -self.y;
    }
    pub const ZERO: Vector = Vector { x: 0.0, y: 0.0 };
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Vector: x = {} , y = {}", self.x, self.y);
    }
}