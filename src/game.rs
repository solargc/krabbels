use crate::action::Action;
use crate::bag::Bag;
use crate::board::Board;
use crate::player::Player;

enum Move {
    PlaceWord,
    ExchangeTiles,
    Pass,
    Challenge,
    Resign,
}

#[derive(Debug)]
pub enum GameEvent {
    TilePlaced {
        row: usize,
        col: usize,
        ch: char,
    },
    WordPlaced {
        word: String,
        score: u32,
        player_id: usize,
    },
    RackUpdated {
        player_id: usize,
        rack: Vec<Option<char>>,
    },
    TurnEnded {
        next_player_id: usize,
    },
}

pub struct Game {
    pub board: Board,
    pub players: Vec<Player>,
    pub bag: Bag,
    // pub current_player_index: usize,
    // pub game_over: bool,
}

#[derive(Debug)]
pub enum MoveError {
    OutOfBounds,
    MustCoverCenter,
    ConflictingLetters,
    Disconnected,
    MissingTiles,
}

impl Game {
    pub fn new() -> Self {
        let mut bag = Bag::new();
        bag.shuffle_bag();
        Self {
            board: Board::new(),
            bag,
            players: Vec::new(),
        }
    }

    pub fn apply(&mut self, action: Action) -> Result<Vec<GameEvent>, MoveError> {
        let events = Vec::new();
        Ok(events)
    }

    pub fn add_player(&mut self, name: String) {
        let id = self.players.len() + 1;
        let mut p = Player::new(id, name);
        p.rack.top_up_from(&mut self.bag);
        self.players.push(p);
    }
}

// on_play_button_click(word_input) {
//     let action = Action::PlaceWord { row, col, dir, word_input };
//     game.apply(action);           // Mutate model
//     let view = game.view();       // Snapshot view
//     ui.render(view);              // Draw on screen
//}
