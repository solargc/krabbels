#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    InvalidFormat,
    EmptyInput,
    InvalidPosition,
    InvalidDirection,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::InvalidFormat => {
                write!(f, "Format invalide")
            }
            InputError::EmptyInput => {
                write!(f, "Entrée vide")
            }
            InputError::InvalidPosition => {
                write!(f, "Position invalide (ex: h8)")
            }
            InputError::InvalidDirection => {
                write!(f, "Direction invalide (h pour horizontal, v pour vertical)")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveError {
    OutOfBounds { row: usize, col: usize },
    CellOccupied { row: usize, col: usize },
    MissingLetter { letter: char },
    WordTooShort,
    NoConnection,
    InvalidWord,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::OutOfBounds { row, col } => {
                let col_letter = (b'a' + *col as u8) as char;
                write!(f, "Hors limites à la position {}{}", col_letter, row + 1)
            }
            MoveError::CellOccupied { row, col } => {
                let col_letter = (b'a' + *col as u8) as char;
                write!(f, "Case occupée à la position {}{}", col_letter, row + 1)
            }
            MoveError::MissingLetter { letter } => {
                write!(f, "Vous n'avez pas la lettre '{}'", letter)
            }
            MoveError::WordTooShort => {
                write!(f, "Le mot doit contenir au moins 2 lettres")
            }
            MoveError::NoConnection => {
                write!(f, "Le mot doit être connecté aux mots existants")
            }
            MoveError::InvalidWord => {
                write!(f, "Mot invalide (pas dans le dictionnaire)")
            }
        }
    }
}
