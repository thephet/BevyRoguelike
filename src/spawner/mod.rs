use crate::prelude::*;
mod template;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<MapBuilder>,
) {
    let player_start = mb.player_start;

    let entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas.atlas.clone(),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)), 
                index: '@' as usize, 
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position { x: player_start.x, y: player_start.y, z: 2 })
        .insert(TileSize::square(1.0))
        .insert(Health{current: 10, max: 20})
        .insert(Player{map_level: 0})
        .insert(Naming("Player".to_string()))
        .insert(FieldOfView::new(8))
        .id();

    mb.entity_occupy_tile(entity, player_start);
}

// max hp, name (like "Orc"), ascii code (like "o")
fn goblin() -> (i32, String, char) {
    (1, "Goblin".to_string(), 'g')
}

fn orc() -> (i32, String, char) {
    (2, "Orc".to_string(), 'o')
}

pub fn spawn_enemies(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<MapBuilder>,
) {
    let mut rng = rand::thread_rng();
    let enemies_start = mb.enemies_start.clone();

    for position in enemies_start {

        let roll = rng.gen_range(1..5);
        match roll {
            1 => spawn_healing_potion(&mut commands, atlas.atlas.clone(), &position),
            2 => spawn_magic_mapper(&mut commands, atlas.atlas.clone(), &position),
            _ => {
                let (hp, name, glyph) = match rng.gen_range(0..4) {
                    0 => orc(),
                    _ => goblin(),
                };
                
                let monster_entity = spawn_enemy(
                    &mut commands, 
                    atlas.atlas.clone(), 
                    TextureAtlasSprite {
                        color: Color::rgb(0.698, 0.094, 0.168),
                        custom_size: Some(Vec2::new(1.0, 1.0)), 
                        index: glyph as usize, 
                        ..Default::default()
                    },
                    &name,
                    hp,
                    &position);
        
                mb.entity_occupy_tile(monster_entity, position);
            }
        }
    }
}


fn spawn_enemy(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    sprite: TextureAtlasSprite,
    name: &String,
    hp: i32,
    position: &Position,
) -> Entity 
{
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas,
            sprite: sprite,
            visibility: Visibility{is_visible:false},
            ..Default::default()
        })
        .insert(Naming(name.clone()))
        .insert(Health{current: hp, max: hp})
        .insert(Position { x: position.x, y: position.y, z: 2 })
        .insert(TileSize::square(1.0))
        .insert(ChasingPlayer)
        .insert(FieldOfView::new(8))
        .insert(Enemy).id()
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
        level = player_q.single().map_level;
        // increase level by 1, because this system gets executed before the post_nextlevel
        level += 1;
    }

    // only spawn amulet if we are in the last level
    if level == 2 {
        let amulet_start = mb.amulet_start;
        commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas.atlas.clone(),
            sprite: TextureAtlasSprite {
                color: Color::GOLD,
                custom_size: Some(Vec2::new(1.0, 1.0)), 
                index: 6, 
                ..Default::default()
            },
            visibility: Visibility{is_visible:false},
            ..Default::default()
        })
        .insert(Naming("Amulet of Yala".to_string()))
        .insert(Position { x: amulet_start.x, y: amulet_start.y, z: 2 })
        .insert(TileSize::square(1.0))
        .insert(Item)
        .insert(AmuletOfYala);
    }
}

fn spawn_healing_potion(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    position: &Position,
) {
    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas,
        sprite: TextureAtlasSprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(1.0, 1.0)), 
            index: 'p' as usize, 
            ..Default::default()
        },
        visibility: Visibility{is_visible:false},
        ..Default::default()
    })
    .insert(Naming("Healing Potion".to_string()))
    .insert(Description("Heals 6 Health Points.".to_string()))
    .insert(Position { x: position.x, y: position.y, z: 2 })
    .insert(TileSize::square(1.0))
    .insert(Item)
    .insert(ProvidesHealing{amount: 6});
}

fn spawn_magic_mapper(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    position: &Position,
) {
    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas,
        sprite: TextureAtlasSprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(1.0, 1.0)), 
            index: 'm' as usize, 
            ..Default::default()
        },
        visibility: Visibility{is_visible:false},
        ..Default::default()
    })
    .insert(Naming("Dungeon Map".to_string()))
    .insert(Description("Reveals all the map tiles.".to_string()))
    .insert(Position { x: position.x, y: position.y, z: 2 })
    .insert(TileSize::square(1.0))
    .insert(Item)
    .insert(ProvidesDungeonMap{});
}

// player, enemies and tiles have position
fn despawn_all_with_position(
    mut commands: Commands, 
    position_q: Query<Entity, With<Position>>,
) {
    for e in position_q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// pre_advance level requires to delete all entities, except the player their items
// set the field of view to dirty so it is re-calculated
fn pre_advance_level(
    mut commands: Commands,
    position_q: Query<Entity, (With<Position>, Without<Player>)>,
    mut fov_q: Query<&mut FieldOfView>
) {
    // remove all the entities with position component except player
    for e in position_q.iter() {
        commands.entity(e).despawn_recursive();
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
    let (player_ent, mut player_pos, mut player) = player_q.single_mut();
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
        .add_system_set(
            SystemSet::on_exit(TurnState::StartScreen)
            .label("spawn_characters")
            .with_system(spawn_player)
            .with_system(spawn_enemies)
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::GameOver)
            .label("despawn_all_gameover")
            .with_system(despawn_all_with_position)
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::Victory)
            .label("despawn_all_victory")
            .with_system(despawn_all_with_position)
        )        
        .add_system_set(
            SystemSet::on_enter(TurnState::NextLevel)
            .label("pre_next_level")
            .with_system(pre_advance_level)
        )
        .add_system_set(
            SystemSet::on_exit(TurnState::NextLevel)
            .label("post_next_level")
            .with_system(post_advance_level)
            .with_system(spawn_amulet_of_yala.before(post_advance_level))
            .with_system(spawn_enemies)
        );
    }
}