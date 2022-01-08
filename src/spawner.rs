use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mb: Res<MapBuilder>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas.atlas.clone(),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)), 
                index: '@' as usize, 
                ..Default::default()
            },
            //TextureAtlasSprite::new('@' as usize),
            ..Default::default()
        })
        .insert(Position { x: mb.player_start.x, y: mb.player_start.y, z: 2 })
        .insert(TileSize::square(1.0))
        .insert(Health{current: 15, max: 20})
        .insert(Alive(true))
        .insert(Player);
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
    mb: Res<MapBuilder>,
) {
    let mut rng = rand::thread_rng();

    for position in &mb.enemies_start {

        let (hp, name, glyph) = match rng.gen_range(0..4) {
            0 => orc(),
            _ => goblin(),
        };
        
        spawn_enemy(
            &mut commands, 
            atlas.atlas.clone(), 
            TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)), 
                index: glyph as usize, 
                ..Default::default()
            },
            &name,
            hp,
            position);
    }
}


fn spawn_enemy(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    sprite: TextureAtlasSprite,
    name: &String,
    hp: i32,
    position: &Position,
) {
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
        .insert(Alive(true))
        .insert(Enemy);
}