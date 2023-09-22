use serde::Deserialize;

use super::FieldState;

#[derive(Deserialize, Debug)]
pub struct Board(pub [FieldState; 9]);
