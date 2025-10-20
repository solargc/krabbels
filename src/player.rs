use crate::bag::{Bag, Tile};

pub struct Player {
    pub id: usize,
    pub name: String,
    pub rack: Rack,
    pub score: u32,
}

impl Player {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id: id,
            name: name,
            rack: Rack::new(),
            score: 0,
        }
    }
}

pub struct Rack {
    pub tiles: Vec<Tile>,
}

impl Rack {
    pub const CAPACITY: usize = 7;

    pub fn new() -> Self {
        Self {
            tiles: Vec::with_capacity(Self::CAPACITY),
        }
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn top_up_from(&mut self, bag: &mut Bag) -> usize {
        let need = Self::CAPACITY.saturating_sub(self.tiles.len());
        let drawn = bag.draw_n(need);
        let added = drawn.len();
        self.tiles.extend(drawn);
        added
    }
}
