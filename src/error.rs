#[derive(Debug, Clone, PartialEq)]
pub enum PlacementError {
    OutOfBounds { row: usize, col: usize },
    CellOccupied { row: usize, col: usize },
    MissingLetter { letter: char },
    InvalidInput,
}

impl std::fmt::Display for PlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlacementError::OutOfBounds { row, col } => {
                write!(f, "Position ({}, {}) is out of bounds", row, col)
            }
            PlacementError::CellOccupied { row, col } => {
                write!(f, "Position ({}, {}) is already occupied", row, col)
            }
            PlacementError::MissingLetter { letter } => {
                write!(f, "Rack does not contain letter '{}'", letter)
            }
            PlacementError::InvalidInput => {
                write!(f, "Invalid input: must be non-empty alphabetic characters")
            }
        }
    }
}
