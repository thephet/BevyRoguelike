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
    pub description: Option<String>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>
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
        commands: &mut Commands,
        atlas: Res<CharsetAsset>,
        level: usize,
        mut mb: &mut ResMut<MapBuilder>,
    ) {
        let mut rng = rand::thread_rng();
        let spawn_points = mb.enemies_start.clone();

        let mut available_entities = Vec::new();
        self.entities.iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0 .. t.frequency {
                    available_entities.push(t);
                }
            });

        spawn_points.iter().for_each(|pos| {
            let target_index = rng.gen_range(0..available_entities.len());
            let entity = available_entities[target_index];
            self.spawn_entity(pos, entity, commands, atlas.atlas.clone(), &mut mb);
        });
    }

    fn spawn_entity(
        &self,
        position: &Position,
        template: &Template,
        commands: &mut Commands,
        atlas: Handle<TextureAtlas>,
        mb: &mut ResMut<MapBuilder>,
    ) {
        let mut entity = commands.spawn((SpriteSheetBundle {
            texture_atlas: atlas,
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                index: template.glyph as usize,
                color: match template.entity_type {
                    EntityType::Item => Color::GREEN,
                    EntityType::Enemy => Color::rgb(0.698, 0.094, 0.168),
                },
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        },
            TileSize::square(1.0),
            Naming(template.name.clone().to_string()),
            Position { x: position.x, y: position.y, z: 2 }
        ));

        match template.entity_type {
            EntityType::Item => {
                let desc = template.description.clone().unwrap(); 
                entity.insert(Item)
                    .insert(Description(desc.to_string()));
            }
            EntityType::Enemy => {
                let hp = template.hp.unwrap();
                entity.insert(Health{current: hp, max: hp})
                    .insert(ChasingPlayer)
                    .insert(FieldOfView::new(6))
                    .insert(Enemy);
                mb.entity_occupy_tile(entity.id(), *position);
            }
        }

        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(provides, n)| {
                match provides.as_str() {
                    "Healing" => { entity.insert(ProvidesHealing{amount: *n}); },
                    "MagicMap" => { entity.insert(ProvidesDungeonMap); },
                    _ => { println!("Warning: we don't know how to provide {}", provides); }
                }
            })
        }

        if let Some(damage) = template.base_damage {
            entity.insert(Damage(damage));
            if template.entity_type == EntityType::Item {
                entity.insert(Weapon);
            }
        }

    }
}
