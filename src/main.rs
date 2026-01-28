mod app;
mod error;
mod game;
mod save;
mod ui;

use std::io;

fn main() -> io::Result<()> {
    save::load_game_state()?;
    app::run();

    Ok(())
}
