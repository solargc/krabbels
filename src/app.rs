use std::io::{self, Write};

use crate::error::{InputError, MoveError};
use crate::game::Game;
use crate::game::action::Action::PlaceWord;
use crate::game::bag::Tile;
use crate::game::board::{Board, Direction, Position, Word};
use crate::game::player::Rack;

fn read_input(prompt: &str) -> Result<String, InputError> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        return Err(InputError::EmptyInput);
    }
    Ok(trimmed)
}

fn read_word(
    board: &Board,
    rack: &Rack,
    pos: &Position,
    dir: &Direction,
) -> Result<Word, InputError> {
    let input = read_input("Votre coup: ")?.to_uppercase();
    if input.is_empty() || !input.chars().all(|ch| ch.is_alphabetic()) {
        return Err(InputError::InvalidFormat);
    }

    let tiles: Vec<Tile> = input
        .chars()
        .map(|ch| Tile {
            letter: ch,
            value: 0,
            is_blank: false,
        })
        .collect();

    Ok(Word { tiles })
}

fn read_position() -> Result<Position, InputError> {
    let input = read_input("Position où commence le mot (ex: h8) : ")?.to_lowercase();

    if input.len() < 2 {
        return Err(InputError::InvalidPosition);
    }

    let (col_char, row_part) = input.split_at(1);
    let col_char = col_char.chars().next().unwrap();

    let ascii_code = col_char as u8;

    if !(b'a'..=b'o').contains(&ascii_code) {
        return Err(InputError::InvalidPosition);
    }

    let col = (ascii_code - b'a') as usize;

    let row = match row_part.parse::<usize>() {
        Ok(n) if (1..=15).contains(&n) => n - 1,
        _ => {
            return Err(InputError::InvalidPosition);
        }
    };

    Ok(Position { row, col })
}

fn read_direction() -> Result<Direction, InputError> {
    let input = read_input("Direction (h/v) : ")?.to_lowercase();
    let direction = match input.as_str() {
        "h" => Direction::Across,
        "v" => Direction::Down,
        _ => {
            return Err(InputError::InvalidDirection);
        }
    };
    Ok(direction)
}

pub fn run() {
    let mut game = Game::new();

    game.add_player("Foufou".to_string());

    println!("");
    println!("{}", game.board);
    println!("{}", game.players[0].rack);

    loop {
        let (pos, dir, word) = loop {
            let pos = loop {
                match read_position() {
                    Ok(pos) => break pos,
                    Err(e) => {
                        println!("Erreur: {}. Réssayez.", e)
                    }
                }
            };

            let dir = loop {
                match read_direction() {
                    Ok(dir) => break dir,
                    Err(e) => {
                        println!("Erreur: {}. Réssayez.", e)
                    }
                }
            };

            match read_word(&game.board, &game.players[0].rack, &pos, &dir) {
                Ok(word) => break (pos, dir, word),
                Err(e) => {
                    println!("Erreur: {}. Réessayez.", e);
                    continue;
                }
            }
        };

        let action = PlaceWord { pos, dir, word };

        match game.apply_move(action) {
            Ok(events) => {
                println!("-> Coup accepté !");
                for e in events {
                    println!("Event: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("-> Coup impossible: {}. Recommencez.", e);
                continue;
            }
        }

        println!("");
        println!("{}", game.board);
        println!("{}", game.players[0].rack);

        println!("{} pioche !", game.players[0].name);
        game.players[0].rack.top_up(&mut game.bag);
        println!("{}", game.players[0].rack);
    }

    // let snapshot = game.view();
    // draw_board(&snapshot.board);
}
