use crate::board;

pub enum Action {
    PlaceWord {
        row: usize,
        col: usize,
        dir: board::Direction,
        word: String,
    },
    ShuffleRack,
    EndTurn,
}
