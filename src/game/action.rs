use super::board;

pub enum Action {
    PlaceWord {
        pos: board::Position,
        dir: board::Direction,
        word: board::Word,
    },
    ShuffleRack,
    EndTurn,
}
