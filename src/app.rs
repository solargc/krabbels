use std::io::{self, Write};

use crate::error::PlacementError;
use crate::game::Game;
use crate::game::action::Action::PlaceWord;
use crate::game::board::{Board, Direction, Position, Word};
use crate::game::player::Rack;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_word(
    board: &Board,
    rack: &Rack,
    pos: &Position,
    dir: &Direction,
) -> Result<Word, PlacementError> {
    let input = read_input("Votre coup: ").to_uppercase();
    if input.is_empty() || !input.chars().all(|ch| ch.is_alphabetic()) {
        return Err(PlacementError::InvalidInput);
    }

    let word_len = input.len();
    for i in 0..word_len {
        let (row, col) = Board::step_towards_dir(pos, dir, i);

        if !board.in_bounds(row, col) {
            return Err(PlacementError::OutOfBounds { row, col });
        }

        if board.cells[row][col].letter.is_some() {
            return Err(PlacementError::CellOccupied { row, col });
        }
    }

    let mut word_tiles = Vec::new();
    let mut available_tiles = rack.tiles.clone();

    for ch in input.chars() {
        if let Some(idx) = available_tiles.iter().position(|tile| tile.letter == ch) {
            word_tiles.push(available_tiles.remove(idx));
        } else {
            return Err(PlacementError::MissingLetter { letter: ch });
        }
    }

    Ok(Word { tiles: word_tiles })
}

fn read_position() -> Option<Position> {
    let input = read_input("Position où commence le mot (ex: h8) : ").to_lowercase();

    if input.len() < 2 {
        return None;
    }

    let (col_char, row_part) = input.split_at(1);
    let col_char = col_char.chars().next().unwrap();

    let ascii_code = col_char as u8;

    if !(b'a'..=b'o').contains(&ascii_code) {
        return None;
    }

    let col = (ascii_code - b'a') as usize;

    let row = match row_part.parse::<usize>() {
        Ok(n) if (1..=15).contains(&n) => n - 1,
        _ => {
            return None;
        }
    };

    Some(Position { row, col })
}

fn read_direction() -> Option<Direction> {
    let input = read_input("Direction (h/v) : ").to_lowercase();
    let direction = match input.as_str() {
        "h" => Direction::Across,
        "v" => Direction::Down,
        _ => {
            return None;
        }
    };
    Some(direction)
}

pub fn run() {
    let mut game = Game::new();

    game.add_player("Foufou".to_string());

    println!("");
    println!("{}", game.board);
    println!("{}", game.players[0].rack);

    loop {
        let input_position = loop {
            if let Some(position) = read_position() {
                break position;
            }
            println!("Position invalide, réessayez.")
        };

        let input_direction = loop {
            if let Some(direction) = read_direction() {
                break direction;
            }
            println!("Direction invalide, réessayez.")
        };

        let input_word = loop {
            if let Ok(word) = read_word(
                &game.board,
                &game.players[0].rack,
                &input_position,
                &input_direction,
            ) {
                break word;
            }
            println!("Coup invalide, réessayez.")
        };

        let action = PlaceWord {
            start_pos: input_position,
            direction: input_direction,
            word: input_word,
        };

        match game.apply(action) {
            Ok(events) => {
                println!("-> Coup accepté !");
                for e in events {
                    println!("Event: {:?}", e);
                }
            }
            Err(_e) => {
                eprintln!("-> Coup impossible.");
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
