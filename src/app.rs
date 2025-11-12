use std::io;

use crate::game::action::Action::PlaceWord;
use crate::game::board::{Direction, Position, Word};
use crate::game::player::Rack;
use crate::game::Game;

fn read_word(rack: &Rack) -> Option<Word> {
    println!("Votre coup:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_uppercase();

    if input.is_empty() || !input.chars().all(|ch| ch.is_alphabetic()) {
        return None;
    }

    let mut word_tiles = Vec::new();
    let mut available_tiles = rack.tiles.clone();

    for ch in input.chars() {
        if let Some(idx) = available_tiles.iter().position(|tile| tile.letter == ch) {
            word_tiles.push(available_tiles.remove(idx));
        } else {
            return None;
        }
    }

    Some(Word { tiles: word_tiles })
}

fn read_position() -> Option<Position> {
    println!("Position où commence le mot? (ex: h8)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_lowercase();

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
    println!("Direction? h/v");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let direction_string = input.trim().to_lowercase();
    let direction = match direction_string.as_str() {
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

    let input_direction = loop {
        if let Some(direction) = read_direction() {
            break direction;
        }
    };

    let input_position = loop {
        if let Some(position) = read_position() {
            break position;
        }
    };

    let input_word = loop {
        if let Some(word) = read_word(&game.players[0].rack) {
            break word;
        }
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

    // let snapshot = game.view();
    // draw_board(&snapshot.board);
}
