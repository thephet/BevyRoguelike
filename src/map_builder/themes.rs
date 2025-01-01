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
    fn tile_to_render(&self, tile_type: TileType) -> Option<Glyph> 
    {
        let glyph_color = Color::srgba(0.3, 0.3, 0.3, 1.0);
        let wall_color = Color::srgba(0.05, 0.05, 0.05, 1.0);
        let floor_color = Color::srgba(0.529, 0.529, 0.529, 1.0);

        match tile_type {
            // index 219 is a full square
            TileType::Floor => Some(Glyph::new_nobkg(219, floor_color)),
            TileType::Wall => Some(Glyph::new('#' as usize, glyph_color, wall_color)),
            TileType::Exit => Some(Glyph::new('>' as usize, glyph_color, floor_color)),
            _ => None,
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Option<Glyph> 
    {
        let glyph_color = Color::srgba(0.105, 0.470, 0.215, 1.0);
        let cell_color = Color::srgba(0.352, 0.682, 0.380, 1.0);

        match tile_type {
            // index 219 is a full square
            TileType::Floor => Some(Glyph::new_nobkg(219, cell_color)),
            TileType::Wall => Some(Glyph::new(30 as usize, glyph_color, cell_color)),
            TileType::Exit => Some(Glyph::new('>' as usize, glyph_color, cell_color)),
            _ => None,
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

pub struct CaveTheme {}

impl MapTheme for CaveTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Option<Glyph> 
    {
        let glyph_color = Color::srgba(0.549, 0.317, 0.039, 1.0);
        let cell_color = Color::srgba(0.749, 0.505, 0.176, 1.0);

        match tile_type {
            // index 219 is a full square
            TileType::Floor => Some(Glyph::new_nobkg(219, cell_color)),
            TileType::Wall => Some(Glyph::new(178 as usize, glyph_color, cell_color)),
            TileType::Exit => Some(Glyph::new('>' as usize, glyph_color, cell_color)),
            _ => None,
        }
    }
}

impl CaveTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}
