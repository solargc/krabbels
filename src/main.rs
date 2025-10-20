mod action;
mod app;
mod bag;
mod board;
mod display;
mod game;
mod player;
mod save;

use ron;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct PlayerData {
    name: String,
}

fn main() -> io::Result<()> {
    let data_dir = PathBuf::from("./data");
    let save_path = data_dir.join("game.ron");
    fs::create_dir_all(&data_dir)?;

    let player = load_or_create_player(&save_path)?;
    println!("Current player: {}", player.name);

    app::run();

    Ok(())
}

fn load_or_create_player(path: &PathBuf) -> io::Result<PlayerData> {
    if !path.exists() {
        println!("No save found. Let's create one.");
        return ask_and_save(path);
    }

    let text = fs::read_to_string(path)?;
    match ron::from_str::<PlayerData>(&text) {
        Ok(p) => {
            println!("Welcome back, {}!", p.name);
            Ok(p)
        }
        Err(e) => {
            eprintln!("Could not parse {}: {e}", path.display());
            println!("Starting fresh.");
            ask_and_save(path)
        }
    }
}

fn ask_and_save(path: &PathBuf) -> io::Result<PlayerData> {
    print!("Enter your name: ");
    io::Write::flush(&mut io::stdout())?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    let player = PlayerData { name: name.clone() };
    let ron_text = ron::ser::to_string_pretty(&player, ron::ser::PrettyConfig::default())
        .expect("serialize PlayerData to RON");
    fs::write(path, ron_text)?;
    println!("Saved player '{name}' to {}", path.display());
    Ok(player)
}
