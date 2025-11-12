use rand::prelude::*;

pub const DISTRIBUTION: [(char, u8, u8); 27] = [
    ('_', 2, 0),
    ('A', 9, 1),
    ('B', 2, 3),
    ('C', 2, 3),
    ('D', 3, 2),
    ('E', 15, 1),
    ('F', 2, 4),
    ('G', 2, 2),
    ('H', 2, 4),
    ('I', 8, 1),
    ('J', 1, 8),
    ('K', 1, 10),
    ('L', 5, 1),
    ('M', 3, 2),
    ('N', 6, 1),
    ('O', 6, 1),
    ('P', 2, 3),
    ('Q', 1, 8),
    ('R', 6, 1),
    ('S', 6, 1),
    ('T', 6, 1),
    ('U', 6, 1),
    ('V', 2, 4),
    ('W', 1, 10),
    ('X', 1, 10),
    ('Y', 1, 10),
    ('Z', 1, 10),
];

#[derive(Copy, Clone)]
pub struct Tile {
    pub letter: char,
    pub value: u8,
    pub is_blank: bool,
}

pub struct Bag {
    pub tiles: Vec<Tile>,
}

impl Bag {
    pub fn new() -> Self {
        let mut tiles = Vec::new();

        for &(ch, count, val) in DISTRIBUTION.iter() {
            for _ in 0..count {
                let is_blank = val == 0;
                tiles.push(Tile {
                    letter: ch,
                    value: val,
                    is_blank,
                });
            }
        }

        Self { tiles }
    }

    pub fn shuffle_bag(&mut self) {
        let mut rng = rand::rng();
        self.tiles.shuffle(&mut rng);
    }

    pub fn draw_n(&mut self, n: usize) -> Vec<Tile> {
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(t) = self.tiles.pop() {
                out.push(t);
            } else {
                break;
            }
        }
        out
    }
}
