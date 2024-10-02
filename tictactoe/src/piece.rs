#[derive(PartialEq)]
pub enum Piece {
    X, // Cross
    O, // Nought
    N, // None
}

impl Piece {
    pub fn get_name(&self) -> String {
        match self {
            Piece::X => String::from("Crosses"),
            Piece::O => String::from("Noughts"),
            Piece::N => String::new(),
        }
    }

    pub fn get_icon(&self) -> &str {
        match self {
            Piece::X => "|X|",
            Piece::O => "|O|",
            Piece::N => "|_|",
        }
    }

    pub fn get_value(&self) -> i8 {
        match self {
            Piece::X => 1,
            Piece::O => -1,
            Piece::N => 0,
        }
    }
}
