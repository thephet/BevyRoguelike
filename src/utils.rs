use std::ops;
use std::cmp;
use crate::prelude::*;

#[derive(Component, Copy, Clone, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x:i32, y:i32, z:i32) -> Self {
        Self{x, y, z}
    }
    pub fn new_from2d(x:i32, y:i32) -> Self {
        Self{x, y, z:0}
    }
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

impl cmp::PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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