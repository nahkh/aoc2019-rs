use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position3 {
    pub fn new(x: i32, y: i32, z: i32) -> Position3 {
        Position3 { x, y, z }
    }
}

impl Add for Position3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Position3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Position3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
