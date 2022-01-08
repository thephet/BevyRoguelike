use std::ops;
use crate::prelude::*;

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

impl From<(Point, i32)> for Position {
    fn from ((point, newz): (Point, i32)) -> Self {
        Position { x:point.x, y:point.y, z:newz }
    }
}

impl From<Position> for Point {
    fn from(item: Position) -> Self {
        Point { x:item.x, y:item.y}
    }
}

// impl<T: Into<Point>> From<T> for Position {
//     fn from(item: T) -> Self {
//         let item = item.into(); //now it's a Point
//         Position { x:item.x, y:item.y, z:0 }
//     }  
// }

// impl<T: Into<Position>> From<T> for Point {
//     fn from(item: T) -> Self {
//         let item = item.into(); //now it's a Point
//         Point { x:item.x, y:item.y }
//     }  
// }