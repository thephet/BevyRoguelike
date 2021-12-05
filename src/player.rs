use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
) {

    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas.atlas.clone(),
        sprite: TextureAtlasSprite::new(64),
        ..Default::default()
    })
    .insert(Position { x: 40, y: 25, z: 1 })
    .insert(TileSize::square(1.0));

}