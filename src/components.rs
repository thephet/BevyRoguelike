use std::collections::HashSet;

use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub map_level: u32
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MapTile;

#[derive(Component, Clone)]
pub struct Naming(pub String);

// used for objects and similar to provide a description about themselves
#[derive(Component, Clone)]
pub struct Description(pub String);

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
pub struct ExitTile;

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
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component)]
pub struct ProvidesDungeonMap;

#[derive(Component)]
pub struct Carried(pub Entity);

#[derive(Component)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity
}