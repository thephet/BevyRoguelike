use crate::prelude::*;

pub struct Player;
pub struct Enemy;
pub struct Alive(pub bool);

#[derive(Clone)]
pub struct Naming(pub String);

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
pub struct ChasingPlayer;

#[derive(Clone, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}

#[derive(Clone, Copy)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity
}

pub struct  Health {
    pub current: i32,
    pub max: i32
}

