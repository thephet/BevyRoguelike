use std::collections::HashSet;

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
pub struct Item;

#[derive(Component)]
pub struct AmuletOfYala;

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
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty:bool
}
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true
        }
    }
    pub fn clone_dirty(&self) -> Self {
        Self{
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true
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

