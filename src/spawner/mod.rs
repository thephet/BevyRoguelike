use crate::prelude::*;
mod template;
use template::Templates;

use bevy::color::palettes::css::GOLD;


pub fn spawn_level(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<MapBuilder>,
    player_q: Query<&Player>,
) {
    // start by getting the player, if it exists, to get the level
    // if it doesnt exist, then it is level 0
    let mut level= 0;
    if player_q.iter().count() > 0 {
        if let Ok(player_ent) = player_q.single() {
            level = player_ent.map_level;
        }
    }
    
    // load template from file and spawn entities
    let template = Templates::load();
    template.spawn_entities(&mut commands, atlas, level as usize, &mut mb);
}

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<MapBuilder>,
) {
    let player_start = mb.player_start;

    let entity = commands
    .spawn((
        Sprite {
            image: atlas.texture.clone(),

            texture_atlas: Some(TextureAtlas {
                layout: atlas.atlas.clone(),
                index: '@' as usize,
            }),

            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        },

        Player { map_level: 0 },
        Naming("Player".to_string()),
        Position { x: player_start.x, y: player_start.y, z: 2 },
        TileSize::square(1.0),
        Health { current: 15, max: 20 },
        FieldOfView::new(8),
        Damage(1),
    ))
    .id();
    mb.entity_occupy_tile(entity, player_start);
}

fn spawn_amulet_of_yala(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mb: Res<MapBuilder>,
    player_q: Query<&Player>,
) {
    // start by getting the player, if it exists, to get the level
    // if it doesnt exist, then it is level 0
    let mut level= 0;
    if player_q.iter().count() > 0 {
        if let Ok(player_ent) = player_q.single() {
            level = player_ent.map_level;
        }
        // increase level by 1, because this system gets executed before the post_nextlevel
        level += 1;
    }

    // only spawn amulet if we are in the last level
    if level == 2 {
        let amulet_start = mb.amulet_start;
        commands
            .spawn((
                // `SpriteBundle` → `Sprite` in 0.15
                Sprite {
                    // `texture` from SpriteBundle → `image` field on Sprite
                    image: atlas.texture.clone(),

                    // `TextureAtlas` is no longer a separate component:
                    // it is now embedded directly in the Sprite
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas.atlas.clone(),
                        index: 6,
                    }),

                    // same color and size as before
                    color: GOLD.into(),
                    custom_size: Some(Vec2::new(1.0, 1.0)),

                    // visibility & transform are auto-added when Sprite is added
                    ..Default::default()
                },

                // Visibility is still its own component in 0.15
                Visibility::Hidden,

                // your own game ECS components remain unchanged
                Item,
                TileSize::square(1.0),
                Position { x: amulet_start.x, y: amulet_start.y, z: 2 },
                Naming("Amulet of Yala".to_string()),
                AmuletOfYala,
            ));

    }
}

// player, enemies and tiles have position
fn despawn_all_with_position(
    mut commands: Commands, 
    position_q: Query<Entity, With<Position>>,
) {
    for e in position_q.iter() {
        commands.entity(e).despawn();
    }
}

// pre_advance level requires to delete all entities, except the player their items
// set the field of view to dirty so it is re-calculated
fn pre_advance_level(
    mut commands: Commands,
    position_q: Query<Entity, (With<Position>, Without<Player>, Without<Carried>)>,
    mut fov_q: Query<&mut FieldOfView>
) {
    // remove all the entities with position component except player
    for e in position_q.iter() {
        commands.entity(e).despawn();
    }

    // set all the fov is_dirty to true, so they will need to be recalculated
    fov_q.iter_mut().for_each(|mut fov| fov.is_dirty = true);
}

// post_advance level sets the location of the player in the new map, advaces its level var
fn post_advance_level(
    mut mb: ResMut<MapBuilder>,
    mut player_q: Query<(Entity, &mut Position, &mut Player)>,
) {
    // get player position from new map
    let player_start = mb.player_start;

    // get player and set its position based on new map and also update map level
    let (player_ent, mut player_pos, mut player) = player_q.single_mut().unwrap();
    player_pos.x = player_start.x;
    player_pos.y = player_start.y;
    player.map_level += 1;
    // also update the map with the occupation info
    mb.entity_occupy_tile(player_ent, player_start);
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems
        (
            OnExit(TurnState::StartScreen),
            (spawn_player, spawn_level)
        )
        
        .add_systems(OnEnter(TurnState::GameOver), despawn_all_with_position)
        .add_systems(OnEnter(TurnState::Victory), despawn_all_with_position)
        
        .add_systems(OnEnter(TurnState::NextLevel), pre_advance_level)
        .add_systems(
            OnExit(TurnState::NextLevel),
            (
                spawn_amulet_of_yala,
                post_advance_level,
                spawn_level
            ).chain()
        );
    }
}