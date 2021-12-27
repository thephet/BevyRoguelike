use crate::prelude::*;

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

#[derive(Clone, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}

pub struct  Health {
    pub current: i32,
    pub max: i32
}