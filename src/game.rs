use crate::board::{Board, Direction};

pub const BOARD_SIZE: usize = 15;

pub struct Game {
    pub board: Board,
    // pub players: Vec<Player>,
    // pub current_player_index: usize,
    // pub bag: Vec<Tile>,
    // pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    pub fn place_word(&mut self, x: usize, y: usize, dir: Direction, word: &str) {
        self.board.place_word(x, y, dir, word);
    }
}
