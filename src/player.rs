use crate::board::Tile;

struct Player {
    pub id: usize,
    pub name: String,
    pub rack: Vec<Tile>,
    pub score: u32,
}
