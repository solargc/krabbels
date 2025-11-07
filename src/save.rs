use ron;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct PlayerData {
    name: String,
}

fn ask_and_save(path: &PathBuf) -> io::Result<PlayerData> {
    print!("Enter your name: ");
    io::Write::flush(&mut io::stdout())?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    let player = PlayerData { name };
    let ron_text = ron::ser::to_string_pretty(&player, ron::ser::PrettyConfig::default())
        .expect("serialize PlayerData to RON");
    fs::write(path, ron_text)?;
    println!("Saved player '{}' to {}", player.name, path.display());
    Ok(player)
}

fn load_data(path: &PathBuf) -> io::Result<PlayerData> {
    if !path.exists() {
        println!("No data found. Let's start fresh!");
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

pub fn load_game_state() -> io::Result<()> {
    let save_dir = PathBuf::from("./data");
    let save_path = save_dir.join("game.ron");

    fs::create_dir_all(&save_dir)?;

    let player = load_data(&save_path)?;
    println!("Current player: {}", player.name);

    Ok(())
}
