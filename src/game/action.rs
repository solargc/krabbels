use super::board;

pub enum Action {
    PlaceWord {
        row: usize,
        col: usize,
        dir: board::Direction,
    },
    ShuffleRack,
    EndTurn,
}
