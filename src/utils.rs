use std::ops;
use bracket_lib::prelude::Point;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Support adding a position to a position
impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(mut self, rhs: Position) -> Position {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl From<Point> for Position {
    fn from(item: Point) -> Self {
        Position { x:item.x, y:item.y, z:0 }
    }
}