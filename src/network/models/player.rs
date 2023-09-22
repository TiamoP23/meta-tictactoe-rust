use serde::Deserialize;

use super::FieldState;

#[derive(Deserialize, Debug)]
pub struct Player {
    pub id: String,
    pub score: u32,
    pub symbol: FieldState,
}
