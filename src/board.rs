pub const BOARD_SIZE: usize = 15;

pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub enum Direction {
    Across,
    Down,
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
}
