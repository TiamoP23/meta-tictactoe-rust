use log::{debug, info};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::{
    game::logic::board,
    network::models::{Board, FieldState, GameRoundEvent, MetaBoard, Position},
};

pub fn round_handler(event: GameRoundEvent) -> Position {
    let position: Position = rand::random();

    position
}
