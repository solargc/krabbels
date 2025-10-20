use crate::action::Action::PlaceWord;
use crate::board::Direction;
use crate::game::Game;

pub fn run() {
    let mut game = Game::new();
    game.add_player("Foufou".to_string());
    println!("{}", game.board);
    println!("{}", game.players[0].rack);

    let action = PlaceWord {
        row: 7,
        col: 7,
        dir: Direction::Across,
        word: "HELLO".into(),
    };

    match game.apply(action) {
        Ok(events) => {
            println!("Move accepted!");
            for e in events {
                println!("Event: {:?}", e);
            }
        }
        Err(_e) => {
            eprintln!("Invalid move");
        }
    }

    // let snapshot = game.view();
    // draw_board(&snapshot.board);
}
