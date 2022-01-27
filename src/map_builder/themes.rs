use crate::prelude::*;

pub struct Glyph {
    // the index in the atlas sprite sheet
    pub index: usize,
    // the color to tint the glyph
    pub color: Color,
    // the background color. If the glyph uses the full cell, not needed
    pub bkg_color: Option<Color>
}
impl Glyph {
    fn new(index: usize, color: Color, bkg_color: Color) -> Self {
        Self{index, color, bkg_color: Some(bkg_color)}
    }
    fn new_nobkg(index: usize, color: Color) -> Self {
        Self{index, color, bkg_color: None}
    }
}

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Option<Glyph> {
        match tile_type {
            // index 219 is a full square
            TileType::Floor => Some(Glyph::new_nobkg(219, Color::rgba(0.529, 0.529, 0.529, 1.0))),
            TileType::Wall => Some(Glyph::new(
                '#' as usize,
                Color::rgba(0.3, 0.3, 0.3, 1.0),
                Color::rgba(0.05, 0.05, 0.05, 1.0))
            ),
            _ => None,
        }
    }
}

// pub struct ForestTheme {}

// impl MapTheme for ForestTheme {
//     fn tile_to_render(&self, tile_type: TileType) -> Option<TextureAtlasSprite> {
//         match tile_type {
//             TileType::Floor => Some(TextureAtlasSprite {
//                                     color: Color::rgba(0.6, 0.847, 0.788, 1.0),
//                                     custom_size: Some(Vec2::new(1.0, 1.0)), 
//                                     index: 219 as usize, // 219 is a full square
//                                     ..Default::default()
//                                 }),
//             TileType::Wall => Some(TextureAtlasSprite {
//                                 color: Color::rgba(0.172, 0.635, 0.372, 1.0),
//                                 custom_size: Some(Vec2::new(1.0, 1.0)), 
//                                 index: 30 as usize, 
//                                 ..Default::default()
//                             }),
//             _ => None,
//         }
//     }
// }

// impl ForestTheme {
//     pub fn new() -> Box<dyn MapTheme> {
//         Box::new(Self{})
//     }
// }
