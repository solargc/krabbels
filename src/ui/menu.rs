use std::io::{self, Write};

use crate::error::InputError;
use crate::game::action::Action;
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

fn read_word() -> Result<Word, InputError> {
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

pub fn prompt_action(board: &Board, rack: &Rack) -> Action {
    loop {
        let pos = loop {
            match read_position() {
                Ok(pos) => break pos,
                Err(e) => println!("Erreur: {}. Réessayez.", e),
            }
        };

        let dir = loop {
            match read_direction() {
                Ok(dir) => break dir,
                Err(e) => println!("Erreur: {}. Réessayez.", e),
            }
        };

        match read_word() {
            Ok(word) => return PlaceWord { pos, dir, word },
            Err(e) => println!("Erreur: {}. Réessayez.", e),
        }
    }
}
