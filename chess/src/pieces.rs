#[derive(Clone, PartialEq)]
pub enum Team {
    Black,
    White,
}
impl Team {
    pub fn get_string(&self) -> String {
        match self {
            Team::Black => String::from("Black"),
            Team::White => String::from("White"),
        }
    }
}

#[derive(Clone)]
pub enum Type {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}
impl Type {
    pub fn get_char(&self) -> char {
        match self {
            Type::Pawn => 'P',
            Type::Knight => 'H',
            Type::Bishop => 'B',
            Type::Rook => 'R',
            Type::Queen => 'Q',
            Type::King => 'K',
        }
    }
}

pub struct Piece {
    /** (x, y) */
    position: (char, usize),
    first: bool,
    team: Team,
    piece: Type,
}

#[allow(dead_code)]
impl Piece {
    pub fn new(position: (char, usize), team: Team, piece: Type) -> Piece {
        Piece {
            position,
            first: true,
            team,
            piece,
        }
    }

    pub fn get_position(&self) -> (char, usize) {
        self.position
    }

    pub fn set_position(&mut self, new_pos: (char, usize)) {
        if self.first {
            self.first = false;
        }
        self.position = new_pos
    }

    pub fn get_piece(&self) -> Type {
        self.piece.clone()
    }

    pub fn get_team(&self) -> Team {
        self.team.clone()
    }

    pub fn get_first_flag(&self) -> bool {
        self.first
    }

    pub fn get_moves(&self) -> Vec<(usize, usize)> {
        match self.piece {
            Type::Bishop => todo!(),
            Type::King => todo!(),
            Type::Knight => todo!(),
            Type::Pawn => todo!(),
            Type::Queen => todo!(),
            Type::Rook => todo!(),
        }
    }
}
