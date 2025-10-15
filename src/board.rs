use crate::game::BOARD_SIZE;

pub struct Tile {
    pub letter: char,
    pub value: u8,
}

pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub enum Direction {
    Across,
    Down,
}

pub struct Board {
    pub cells: [[Option<char>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
    pub fn place_word(&mut self, x: usize, y: usize, dir: Direction, word: &str) {
        for (i, ch) in word.chars().enumerate() {
            let (tx, ty) = match dir {
                Direction::Across => (x + i, y),
                Direction::Down => (x, y + i),
            };
            if tx < BOARD_SIZE && ty < BOARD_SIZE {
                self.cells[ty][tx] = Some(ch.to_ascii_uppercase());
            }
        }
    }
}
