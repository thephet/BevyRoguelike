//use crate::prelude::*;

pub struct Player;

pub struct Enemy;

pub struct MainCamera;

pub struct TileSize {
    pub width: f32,
    pub height: f32,
}
impl TileSize {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

pub struct MovingRandomly;