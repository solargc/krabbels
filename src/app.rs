use crate::board::Direction;
use crate::game::Game;

pub fn run() {
    let mut game = Game::new();
    game.place_word(7, 7, Direction::Across, "HELLO");
    game.place_word(7, 7, Direction::Down, "WorLd");
    game.place_word(0, 1, Direction::Down, "Coucou");
    game.place_word(9, 10, Direction::Across, "Foufou");
    println!("{}", game.board);
}
