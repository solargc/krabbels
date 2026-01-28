use ron;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlayerData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SaveFile {
    created_at_unix: i64,
    player: PlayerData,
}

fn prompt_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn ensure_dir(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

fn list_saves(save_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();

    if !save_dir.exists() {
        return Ok(files);
    }

    for entry in fs::read_dir(save_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("ron") {
            files.push(path);
        }
    }

    files.sort();
    files.reverse();

    Ok(files)
}

fn save_to_path(path: &Path, save: &SaveFile) -> io::Result<()> {
    let ron_text = ron::ser::to_string_pretty(save, ron::ser::PrettyConfig::default())
        .expect("serialize SaveFile to RON");
    fs::write(path, ron_text)?;
    Ok(())
}

fn load_from_path(path: &Path) -> io::Result<SaveFile> {
    let text = fs::read_to_string(path)?;
    ron::from_str::<SaveFile>(&text)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e}")))
}

fn create_new_game(save_dir: &Path) -> io::Result<(PathBuf, SaveFile)> {
    let name = prompt_line("Enter your name: ")?;
    let save = SaveFile {
        created_at_unix: now_unix(),
        player: PlayerData { name },
    };

    let filename = format!("game_{}.ron", save.created_at_unix);
    let path = save_dir.join(filename);

    save_to_path(&path, &save)?;
    println!("Created new game: {}", path.display());

    Ok((path, save))
}

fn choose_or_create_game(save_dir: &Path) -> io::Result<(PathBuf, SaveFile)> {
    let saves = list_saves(save_dir)?;

    if saves.is_empty() {
        println!("No saved games found. Starting fresh!");
        return create_new_game(save_dir);
    }

    let shown = saves.len().min(5);

    let mut entries: Vec<(PathBuf, Result<SaveFile, String>)> = Vec::with_capacity(shown);

    for path in saves.iter().take(shown) {
        let loaded = match load_from_path(path) {
            Ok(save) => Ok(save),
            Err(e) => Err(format!("{e}")),
        };
        entries.push((path.clone(), loaded));
    }

    println!("Saved games:");
    for (i, (path, loaded)) in entries.iter().enumerate() {
        let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("???");

        match loaded {
            Ok(save) => {
                // For now: single player name
                println!("  {}) {} — player: {}", i + 1, fname, save.player.name);
            }
            Err(_) => {
                // Don't spam the full parse error in the menu; keep it readable
                println!("  {}) {} — (could not load)", i + 1, fname);
            }
        }
    }
    println!("  n) Start a new game");

    loop {
        let choice = prompt_line("Choose a save (1..), or 'n': ")?;

        if choice.eq_ignore_ascii_case("n") {
            return create_new_game(save_dir);
        }

        if let Ok(idx) = choice.parse::<usize>() {
            if idx >= 1 && idx <= shown {
                let (path, loaded) = &entries[idx - 1];

                match loaded {
                    Ok(save) => {
                        println!(
                            "Loaded game: {} (player: {})",
                            path.display(),
                            save.player.name
                        );
                        return Ok((path.clone(), save.clone()));
                    }
                    Err(e) => {
                        eprintln!("Could not load {}: {e}", path.display());
                        println!("Pick another save or choose 'n' for new.");
                    }
                }
            }
        }

        println!("Invalid choice.");
    }
}

pub fn load_game_state() -> io::Result<()> {
    let data_dir = PathBuf::from("./data");

    ensure_dir(&data_dir)?;

    let (_path, save) = choose_or_create_game(&data_dir)?;

    println!("Current player: {}", save.player.name);

    Ok(())
}
