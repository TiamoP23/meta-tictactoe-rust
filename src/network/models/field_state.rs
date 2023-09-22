use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum FieldState {
    O,
    X,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "-")]
    Draw,
}
