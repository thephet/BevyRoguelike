use crate::prelude::*;

// resource type
#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    Setup,
    StartScreen,
    AwaitingInput,
    InMenus,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PopUpState {
    #[default]
    None,
    EquipmentPopup,
    InventoryPopup,
}

#[derive(Resource)]
pub struct GameLog {
    pub entries : Vec<String>
}

impl GameLog {

    pub fn new() -> Self {

        let mut log: Vec<String> = Vec::with_capacity(4);
        log.push("Log...\n".to_string());
        log.push("Use the arrow keys to move.\n".to_string());
        log.push("Bump into the enemies to attack them.\n".to_string());
        log.push("Find the amulet to win the game.\n".to_string());

        let entries_log = GameLog{
            entries: log
        };
        entries_log
    }

    pub fn add_entry(&mut self, message:String) {
        self.entries.push(message);
        self.entries.remove(0);
    }
}