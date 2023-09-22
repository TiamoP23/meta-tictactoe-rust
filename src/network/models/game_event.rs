use serde::Deserialize;

use super::{Board, GameDetails, MetaBoard};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "INIT")]
    Init(GameInitEvent),
    #[serde(rename = "RESULT")]
    Result(GameRoundEvent),
    #[serde(rename = "ROUND")]
    Round(GameRoundEvent),
}

#[derive(Deserialize, Debug)]
pub struct GameInitEvent {
    #[serde(flatten)]
    pub details: GameDetails,
}

#[derive(Deserialize, Debug)]
pub struct GameRoundEvent {
    #[serde(flatten)]
    pub details: GameDetails,
    pub board: MetaBoard,
    pub overview: Board,
    #[serde(rename = "forcedSection")]
    pub forced_section: Option<usize>,
}
