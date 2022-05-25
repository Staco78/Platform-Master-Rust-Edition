use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
}

impl std::ops::Add<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn sub(self, other: Vec2i) -> Vec2i {
        Vec2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<i32> for Vec2i {
    type Output = Vec2i;

    fn mul(self, other: i32) -> Vec2i {
        Vec2i {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Into<Vec2> for Vec2i {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}
