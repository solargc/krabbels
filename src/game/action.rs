use super::board;

pub enum Action {
    PlaceWord {
        start_pos: board::Position,
        dir: board::Direction,
    },
    ShuffleRack,
    EndTurn,
}
