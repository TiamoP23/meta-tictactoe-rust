use log::info;

use crate::network::models::{GameInitEvent, GameRoundEvent};

pub fn init_handler(event: GameInitEvent) {
    let game_id = event.details.id;

    info!("Started game {}!", game_id);
}
