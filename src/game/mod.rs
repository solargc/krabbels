pub mod action;
pub mod bag;
pub mod board;
pub mod player;

use self::action::Action;
use self::bag::Bag;
use self::board::Board;
use self::player::{Player, Rack};

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
        let mut events = Vec::new();

        match action {
            Action::PlaceWord { start_pos, dir } => {
                let count =
                    self.board
                        .try_place_tiles(&mut self.players[0].rack, &start_pos, &dir, 7);

                if count == 0 {
                    return Err(MoveError::OutOfBounds);
                }

                //events.push(GameEvent::WordPlaced { count });
            }
            _ => {}
        }

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
