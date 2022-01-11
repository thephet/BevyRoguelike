use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;


#[derive(Component, Clone)]
pub struct Naming(pub String);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
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

#[derive(Component)]
pub struct MovingRandomly;
#[derive(Component)]
pub struct ChasingPlayer;

#[derive(Component, Clone, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}

#[derive(Component, Clone, Copy)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity
}

#[derive(Component)]
pub struct  Health {
    pub current: i32,
    pub max: i32
}

