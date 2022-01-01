use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mb: Res<MapBuilder>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas.atlas.clone(),
            sprite: TextureAtlasSprite::new('@' as u32),
            ..Default::default()
        })
        .insert(Position { x: mb.player_start.x, y: mb.player_start.y, z: 1 })
        .insert(TileSize::square(1.0))
        .insert(Health{current: 20, max: 30})
        .insert(Player);
}


pub fn spawn_enemies(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mb: Res<MapBuilder>,
) {
    let mut rng = rand::thread_rng();

    for position in &mb.enemies_start {
        spawn_enemy(
            &mut commands, 
            atlas.atlas.clone(), 
            match rng.gen_range(0..4) {
                0 => TextureAtlasSprite::new('E' as u32),
                1 => TextureAtlasSprite::new('O' as u32),
                2 => TextureAtlasSprite::new('o' as u32),
                _ => TextureAtlasSprite::new('g' as u32),
            },
            position);
    }
}


fn spawn_enemy(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    sprite: TextureAtlasSprite,
    position: &Position,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas,
            sprite: sprite,
            ..Default::default()
        })
        .insert(Position { x: position.x, y: position.y, z: 1 })
        .insert(TileSize::square(1.0))
        .insert(MovingRandomly)
        .insert(Enemy);
}