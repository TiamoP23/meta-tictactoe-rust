use log::info;

use crate::network::models::Position;

use crate::network::models::GameEvent;

use super::handler::{init_handler, result_handler, round_handler};

pub fn handle_game_event(event: GameEvent) -> Option<Position> {
    match event {
        GameEvent::Init(event) => {
            init_handler(event);
            None
        }
        GameEvent::Round(event) => {
            let position = round_handler(event);

            Some(position)
        }
        GameEvent::Result(event) => {
            result_handler(event);
            None
        }
    }
}
