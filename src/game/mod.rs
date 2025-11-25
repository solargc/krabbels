pub mod action;
pub mod bag;
pub mod board;
pub mod player;

use self::action::Action;
use self::bag::Bag;
use self::board::Board;
use self::player::Player;
use crate::error::MoveError;

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
        let mut events = Vec::new();

        match action {
            Action::PlaceWord { pos, dir, word } => {
                self.board.validate_bounds(&pos, &dir, &word)?;
                self.board.validate_cells_available(&pos, &dir, &word)?;
                self.board
                    .validate_player_has_tiles(&self.players[0].rack, &word)?;
                self.board
                    .place_word(&mut self.players[0].rack, &pos, &dir, &word);

                //events.push(GameEvent::WordPlaced { count });
            }
            _ => {}
        }

        Ok(events)
    }

    pub fn add_player(&mut self, name: String) {
        let id = self.players.len() + 1;
        let mut player = Player::new(id, name);
        player.rack.top_up(&mut self.bag);
        self.players.push(player);
    }
}

// on_play_button_click(word_input) {
//     let action = Action::PlaceWord { row, col, dir, word_input };
//     game.apply(action);           // Mutate model
//     let view = game.view();       // Snapshot view
//     ui.render(view);              // Draw on screen
//}
