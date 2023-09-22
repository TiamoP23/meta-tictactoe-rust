use serde::Deserialize;

use super::Board;

#[derive(Deserialize, Debug)]
pub struct MetaBoard(pub [Board; 9]);
