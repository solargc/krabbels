use crate::{error::MoveError, game::bag::Tile};

use super::player::Rack;

pub const BOARD_SIZE: usize = 15;

#[derive(Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub enum Direction {
    Across,
    Down,
}

pub struct Word {
    pub tiles: Vec<Tile>,
}

#[derive(Copy, Clone)]
pub enum CellKind {
    Normal,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub letter: Option<char>,
    pub kind: CellKind,
}

pub struct Board {
    pub cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        let mut cells = [[Cell {
            letter: None,
            kind: CellKind::Normal,
        }; BOARD_SIZE]; BOARD_SIZE];

        let mut set_kind = |positions: &[(usize, usize)], kind: CellKind| {
            for &(r, c) in positions {
                cells[r][c].kind = kind;
            }
        };

        const TW: &[(usize, usize)] = &[
            (0, 0),
            (0, 7),
            (0, 14),
            (7, 0),
            (7, 14),
            (14, 0),
            (14, 7),
            (14, 14),
        ];

        const DW: &[(usize, usize)] = &[
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (1, 13),
            (2, 12),
            (3, 11),
            (4, 10),
            (10, 4),
            (11, 3),
            (12, 2),
            (13, 1),
            (10, 10),
            (11, 11),
            (12, 12),
            (13, 13),
            (7, 7),
        ];

        const TL: &[(usize, usize)] = &[
            (1, 5),
            (1, 9),
            (5, 1),
            (5, 5),
            (5, 9),
            (5, 13),
            (9, 1),
            (9, 5),
            (9, 9),
            (9, 13),
            (13, 5),
            (13, 9),
        ];

        const DL: &[(usize, usize)] = &[
            (0, 3),
            (0, 11),
            (2, 6),
            (2, 8),
            (3, 0),
            (3, 7),
            (3, 14),
            (6, 2),
            (6, 6),
            (6, 8),
            (6, 12),
            (7, 3),
            (7, 11),
            (8, 2),
            (8, 6),
            (8, 8),
            (8, 12),
            (11, 0),
            (11, 7),
            (11, 14),
            (12, 6),
            (12, 8),
            (14, 3),
            (14, 11),
        ];

        set_kind(TW, CellKind::TripleWord);
        set_kind(DW, CellKind::DoubleWord);
        set_kind(TL, CellKind::TripleLetter);
        set_kind(DL, CellKind::DoubleLetter);

        Self { cells }
    }

    fn is_empty(&self) -> bool {
        self.cells
            .iter()
            .flatten()
            .all(|cell| cell.letter.is_none())
    }

    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < BOARD_SIZE && col < BOARD_SIZE
    }

    fn step_towards_dir(pos: &Position, dir: &Direction, i: usize) -> (usize, usize) {
        match dir {
            Direction::Across => (pos.row, pos.col + i),
            Direction::Down => (pos.row + i, pos.col),
        }
    }

    pub fn validate_in_bounds(
        &self,
        pos: &Position,
        dir: &Direction,
        word: &Word,
    ) -> Result<(), MoveError> {
        for i in 0..word.tiles.len() {
            let (row, col) = Self::step_towards_dir(pos, dir, i);

            if !self.in_bounds(row, col) {
                return Err(MoveError::OutOfBounds { row, col });
            }
        }
        Ok(())
    }

    pub fn validate_player_has_tiles(&self, rack: &Rack, word: &Word) -> Result<(), MoveError> {
        let mut available_tiles = rack.tiles.clone();

        for tile in &word.tiles {
            if let Some(idx) = available_tiles.iter().position(|t| t.letter == tile.letter) {
                available_tiles.remove(idx);
            } else {
                return Err(MoveError::MissingLetter {
                    letter: tile.letter,
                });
            }
        }

        Ok(())
    }

    pub fn validate_adjacent_tiles(
        &self,
        pos: &Position,
        dir: &Direction,
        word: &Word,
    ) -> Result<(), MoveError> {
        if self.is_empty() {
            for i in 0..word.tiles.len() {
                let (row, col) = Self::step_towards_dir(pos, dir, i);
                if row == 7 && col == 7 {
                    return Ok(());
                }
            }
            return Err(MoveError::MustCoverCenter);
        }

        for i in 0..word.tiles.len() {
            let (row, col) = Self::step_towards_dir(pos, dir, i);

            if self.cells[row][col].letter.is_some() {
                return Ok(());
            }

            if row > 0 && self.cells[row - 1][col].letter.is_some() {
                return Ok(());
            }

            if row < BOARD_SIZE - 1 && self.cells[row + 1][col].letter.is_some() {
                return Ok(());
            }

            if col > 0 && self.cells[row][col - 1].letter.is_some() {
                return Ok(());
            }

            if col < BOARD_SIZE - 1 && self.cells[row][col + 1].letter.is_some() {
                return Ok(());
            }
        }
        Err(MoveError::NoConnection)
    }

    fn is_cell_available(
        &self,
        word: &Word,
        target_letter: char,
        i: usize,
        row: usize,
        col: usize,
    ) -> Result<bool, MoveError> {
        match self.cells[row][col].letter {
            Some(existing_letter) => {
                if existing_letter != target_letter {
                    return Err(MoveError::LetterMismatch { row, col });
                }
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub fn place_word(
        &mut self,
        rack: &mut Rack,
        pos: &Position,
        dir: &Direction,
        word: &Word,
    ) -> Result<(), MoveError> {
        for i in 0..word.tiles.len() {
            let target_letter = word.tiles[i].letter;

            let (row, col) = Self::step_towards_dir(pos, dir, i);

            if self.is_cell_available(word, target_letter, i, row, col)? {
                continue;
            }

            let index = rack
                .tiles
                .iter()
                .position(|tile| tile.letter == target_letter)
                .ok_or(MoveError::MissingLetter {
                    letter: target_letter,
                })?;
            let tile = rack.tiles.remove(index);
            self.cells[row][col].letter = Some(tile.letter);
        }
        Ok(())
    }
}
