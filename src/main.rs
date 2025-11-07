mod app;
mod display;
mod game;
mod save;

use std::io;

fn main() -> io::Result<()> {
    save::load_game_state()?;
    app::run();

    Ok(())
}
