use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn neighbors(&self) -> [Position; 4] {
        [
            *self + Position::new(-1, 0),
            *self + Position::new(1, 0),
            *self + Position::new(0, -1),
            *self + Position::new(0, 1),
        ]
    }

    fn manhattan_magnitude(self) -> u64 {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }

    pub fn manhattan_distance(self, other: Self) -> u64 {
        (self - other).manhattan_magnitude()
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.x.cmp(&self.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
