use ron::{de::from_str, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Save {
    pub version: u32,
    pub players: Vec<String>,
}

impl Save {
    pub fn new() -> Self {
        Self {
            version: 1,
            players: vec![],
        }
    }
}
pub fn load_or_init(path: &Path) -> Save {
    match fs::read_to_string(path) {
        Ok(text) => match from_str::<Save>(&text) {
            Ok(save) => save,
            Err(err) => {
                eprintln!("Save parse error ({err}); starting fresh.");
                Save::new()
            }
        },
        Err(_) => {
            // No file yet — create an empty one.
            let save = Save::new();
            let _ = save_to_path(&save, path);
            save
        }
    }
}

pub fn save_to_path(save: &Save, path: &Path) -> std::io::Result<()> {
    let ron = ron::ser::to_string_pretty(save, PrettyConfig::default())
        .expect("serializing Save to RON never fails for this struct");
    fs::write(path, ron)
}
