use crate::board::Board;
use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some(c) => write!(f, "{}  ", c)?,
                    None => write!(f, ".  ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
