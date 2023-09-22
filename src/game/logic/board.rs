use crate::network::models::{Board, FieldState, Position};

impl Board {
    pub fn get_opponent_fields(&self, own_symbol: &FieldState) -> Vec<usize> {
        let Board(fields) = self;

        let opponent_symbol = match own_symbol {
            FieldState::O => &FieldState::X,
            FieldState::X => &FieldState::O,
            _ => panic!("Invalid player symbol"),
        };

        fields
            .iter()
            .enumerate()
            .filter(|(_, field)| *field == opponent_symbol)
            .map(|(index, _)| index)
            .collect()
    }

    pub fn get_own_fields(&self, own_symbol: &FieldState) -> Vec<usize> {
        let Board(fields) = self;

        fields
            .iter()
            .enumerate()
            .filter(|(_, field)| *field == own_symbol)
            .map(|(index, _)| index)
            .collect()
    }

    pub fn get_empty_fields(&self, own_symbol: &FieldState) -> Vec<usize> {
        let Board(fields) = self;

        fields
            .iter()
            .enumerate()
            .filter(|(_, field)| *field == &FieldState::Empty)
            .map(|(index, _)| index)
            .collect()
    }
}
