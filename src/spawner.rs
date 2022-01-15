use crate::prelude::*;

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
        .insert(Health{current: 20, max: 20})
        .insert(Player)
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
            ..Default::default()
        })
        .insert(Naming(name.clone()))
        .insert(Health{current: hp, max: hp})
        .insert(Position { x: position.x, y: position.y, z: 2 })
        .insert(TileSize::square(1.0))
        .insert(ChasingPlayer)
        .insert(FieldOfView::new(6))
        .insert(Enemy).id()
}

fn spawn_amulet_of_yala(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mb: Res<MapBuilder>,
) {
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
        ..Default::default()
    })
    .insert(Naming("Amulet of Yala".to_string()))
    .insert(Position { x: amulet_start.x, y: amulet_start.y, z: 2 })
    .insert(TileSize::square(1.0))
    .insert(Item)
    .insert(AmuletOfYala);
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

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_exit(TurnState::StartScreen)
            .label("spawn_character")
            .with_system(spawn_player)
            .with_system(spawn_enemies)
            .with_system(spawn_amulet_of_yala)
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::GameOver)
            .label("despawn_all")
            .with_system(despawn_all_with_position)
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::Victory)
            .label("despawn_all")
            .with_system(despawn_all_with_position)
        );
    }
}