use super::board;

pub enum Action {
    PlaceWord {
        start_pos: board::Position,
        direction: board::Direction,
        word: board::Word,
    },
    ShuffleRack,
    EndTurn,
}
