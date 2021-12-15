use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mb: Res<MapBuilder>,
) {

    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas.atlas.clone(),
        sprite: TextureAtlasSprite::new(64),
        ..Default::default()
    })
    .insert(Position { x: mb.player_start.x, y: mb.player_start.y, z: 1 })
    .insert(TileSize::square(1.0))
    .insert(Player);
}