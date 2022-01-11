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
        .insert(Health{current: 15, max: 20})
        .insert(Player)
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
        .insert(Enemy).id()
}