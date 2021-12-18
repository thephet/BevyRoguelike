use crate::prelude::*;

// resource type
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}

// not a resource in bevy but hands on defines it as resource. We will use Bevy State
#[derive(Clone, Eq, PartialEq, Hash, Debug )]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}