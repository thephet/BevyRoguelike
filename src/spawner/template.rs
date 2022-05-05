use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy, Item
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {

    pub fn load() -> Self 
    {
        let file = File::open("assets/template.ron")
            .expect("Failed opening file.");
        from_reader(file).expect("Unable to load templates.")
    }

    pub fn spawn_entities(
        &self,
        level: usize,
        spawn_points: &[Position]
    ) {
        let mut rng = rand::thread_rng();

        let mut available_entities = Vec::new();
        self.entities.iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0 .. t.frequency {
                    available_entities.push(t);
                }
            });

        spawn_points.iter().for_each(|pt| {
            let target_index = rng.gen_range(0..available_entities.len());
            if let entity = available_entities[target_index].clone() {
                self.spawn_entity(pt, entity);
            }
        })
    }
}

impl Template {

    fn spawn_entity(
        self,
        pt: &Point,
        template: &Template,
        mut commands: Commands,
        atlas: Res<CharsetAsset>,
    ) {
        let entity = commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas.atlas.clone(),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)), 
                index: '@' as usize, 
                ..Default::default()
            },
            ..Default::default()
        });
    }
}