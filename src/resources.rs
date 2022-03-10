use crate::prelude::*;

// resource type
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}

// not a resource in bevy but hands on defines it as resource. We will use Bevy State
#[derive(Clone, Eq, PartialEq, Hash, Debug )]
pub enum TurnState {
    StartScreen,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
    //EquipmentPopup,
    InventoryPopup,
}

pub struct GameLog {
    pub entries : Vec<String>
}

impl GameLog {

    pub fn new() -> Self {

        let mut log: Vec<String> = Vec::with_capacity(4);
        log.push("Log...".to_string());
        log.push("\nUse the arrow keys to move.".to_string());
        log.push("\nBump into the enemies to attack them.".to_string());
        log.push("\nFind the amulet to win the game.".to_string());

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