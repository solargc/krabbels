use crate::game::bag::Bag;
use crate::game::board::{Board, CellKind, BOARD_SIZE};
use crate::game::player::Rack;
use colored::*;
use std::collections::BTreeMap;
use std::fmt;

const CELL_W: usize = 4;

fn center(text: &str, width: usize) -> String {
    let visual_len = strip_ansi_len(text);
    let padding = width.saturating_sub(visual_len);
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;
    format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
}

fn strip_ansi_len(text: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for ch in text.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape && ch == 'm' {
            in_escape = false;
        } else if !in_escape {
            len += 1;
        }
    }
    len
}

fn col_labels() -> [char; BOARD_SIZE] {
    let mut labels = [' '; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        labels[i] = (b'A' + i as u8) as char;
    }
    labels
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cols = col_labels();
        write!(f, "     ")?;
        for &c in &cols {
            write!(f, "{}", center(&c.to_string(), CELL_W + 1))?;
        }
        writeln!(f)?;

        write!(f, "    ")?;
        write!(f, "┌")?;
        for x in 0..BOARD_SIZE {
            write!(f, "{}", "─".repeat(CELL_W))?;
            write!(f, "{}", if x + 1 == BOARD_SIZE { "┐" } else { "┬" })?;
        }
        writeln!(f)?;

        for r in 0..BOARD_SIZE {
            write!(f, "{:>3} ", r + 1)?;

            write!(f, "│")?;
            for c in 0..BOARD_SIZE {
                let cell = &self.cells[r][c];
                let label = match cell.letter {
                    Some(ch) => ch.to_ascii_uppercase().to_string().bold().to_string(),
                    None => {
                        if r == 7 && c == 7 {
                            "★".to_string()
                        } else {
                            match cell.kind {
                                CellKind::Normal => " ".to_string(),
                                CellKind::DoubleLetter => "LD".blue().to_string(),
                                CellKind::TripleLetter => "LT".cyan().to_string(),
                                CellKind::DoubleWord => "MD".red().to_string(),
                                CellKind::TripleWord => "MT".magenta().to_string(),
                            }
                        }
                    }
                };
                write!(f, "{}", center(&label, CELL_W))?;
                write!(f, "│")?;
            }
            writeln!(f)?;

            write!(f, "    ")?;
            if r + 1 == BOARD_SIZE {
                write!(f, "└")?;
                for x in 0..BOARD_SIZE {
                    write!(f, "{}", "─".repeat(CELL_W))?;
                    write!(f, "{}", if x + 1 == BOARD_SIZE { "┘" } else { "┴" })?;
                }
                writeln!(f)?;
            } else {
                write!(f, "├")?;
                for x in 0..BOARD_SIZE {
                    write!(f, "{}", "─".repeat(CELL_W))?;
                    write!(f, "{}", if x + 1 == BOARD_SIZE { "┤" } else { "┼" })?;
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut counts: BTreeMap<char, (u8, u32)> = BTreeMap::new();

        for tile in &self.tiles {
            let entry = counts.entry(tile.letter).or_insert((tile.value, 0));
            entry.1 += 1;
        }

        writeln!(f, "--- Bag contents ---")?;
        writeln!(f, "{:<3} | {:<5} | {:<5}", "Let", "Val", "Count")?;
        writeln!(f, "--------------------")?;

        for (letter, (value, count)) in counts {
            writeln!(f, "{:<3} | {:<5} | {:<5}", letter, value, count)?;
        }

        writeln!(f, "Total tiles: {}", self.tiles.len())
    }
}

fn superscript_num(n: u8) -> String {
    const SUP: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    if n == 0 {
        return SUP[0].to_string();
    }
    let mut digits = Vec::new();
    let mut x = n as usize;
    while x > 0 {
        digits.push(SUP[x % 10]);
        x /= 10;
    }
    digits.reverse();
    digits.iter().collect()
}

impl fmt::Display for Rack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chevalet:")?;
        if self.tiles.is_empty() {
            return writeln!(f, "(empty rack)");
        }

        let ascii_fallback = f.alternate();
        let cell_w = 6;
        let tile_count = self.tiles.len();

        write!(f, "┌")?;
        for i in 0..tile_count {
            write!(f, "{}", "─".repeat(cell_w))?;
            write!(f, "{}", if i + 1 == tile_count { "┐" } else { "┬" })?;
        }
        writeln!(f)?;

        write!(f, "│")?;
        for (_, tile) in self.tiles.iter().enumerate() {
            let ch = if tile.value == 0 {
                '_'
            } else {
                tile.letter.to_ascii_uppercase()
            };

            let score = if tile.value == 0 {
                String::new()
            } else if ascii_fallback {
                format!("^{}", tile.value)
            } else {
                superscript_num(tile.value)
            };

            let label = format!("{}{}", ch, score);
            let pad_total = cell_w - label.chars().count();
            let left_pad = pad_total / 2;
            let right_pad = pad_total - left_pad;
            write!(
                f,
                "{}{}{}",
                " ".repeat(left_pad),
                label,
                " ".repeat(right_pad)
            )?;

            write!(f, "│")?;
        }
        writeln!(f)?;

        write!(f, "└")?;
        for i in 0..tile_count {
            write!(f, "{}", "─".repeat(cell_w))?;
            write!(f, "{}", if i + 1 == tile_count { "┘" } else { "┴" })?;
        }
        writeln!(f)?;

        Ok(())
    }
}
