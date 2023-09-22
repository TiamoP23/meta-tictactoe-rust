use crate::network::models::{GameDetails, GameRoundEvent, Player};

impl GameDetails {
    pub fn get_self(&self) -> &Player {
        self.players
            .iter()
            .find(|p| p.id == self.self_id)
            .unwrap()
            .clone()
    }

    pub fn get_opponent(&self) -> &Player {
        self.players
            .iter()
            .find(|p| p.id != self.self_id)
            .unwrap()
            .clone()
    }
}
