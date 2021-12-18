use std::ops;

// from bracket-lib
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Room {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Room {
    // Create a new Room, specifying X/Y Width/Height
    pub fn with_size<T>(x: T, y: T, w: T, h: T) -> Room
    where
        T: TryInto<i32>,
    {
        let x_i32: i32 = x.try_into().ok().unwrap();
        let y_i32: i32 = y.try_into().ok().unwrap();
        Room {
            x1: x_i32,
            y1: y_i32,
            x2: x_i32 + w.try_into().ok().unwrap(),
            y2: y_i32 + h.try_into().ok().unwrap(),
        }
    }
    // Create a new Room, specifying exact dimensions
    pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> Room
    where
        T: TryInto<i32>,
    {
        Room {
            x1: x1.try_into().ok().unwrap(),
            y1: y1.try_into().ok().unwrap(),
            x2: x2.try_into().ok().unwrap(),
            y2: y2.try_into().ok().unwrap(),
        }
    }
    // Returns true if this overlaps with other,
    pub fn intersect(&self, other: &Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    // Returns the center of the Room
    pub fn center(&self) -> Position {
        Position{x: (self.x1 + self.x2) / 2, y: (self.y1 + self.y2) / 2, z: 0}
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Position),
    {
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                f(Position{x, y, z:0});
            }
        }
    }
}


#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
/// Support adding a point to a point
impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(mut self, rhs: Position) -> Position {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}