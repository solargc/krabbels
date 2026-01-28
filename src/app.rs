use crate::game::Game;
use crate::ui::{display, menu};

pub fn run() {
    let mut game = Game::new();
    game.add_player("Foufou".to_string());

    display::show_game(&game);

    loop {
        let action = menu::prompt_action(&game.board, &game.players[0].rack);

        match game.apply_move(action) {
            Ok(events) => {
                display::show_events(&events);
            }
            Err(e) => {
                display::show_move_error(&e);
                continue;
            }
        }

        display::show_game(&game);

        println!("{} pioche !", game.players[0].name);
        game.players[0].rack.top_up(&mut game.bag);
        println!("{}", game.players[0].rack);

        // later:
        // save::io::save_to_path(&session.path, &session.save)?;
    }
}
