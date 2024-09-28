use crate::pieces::{Piece, Team, Type};

/** chess board is 8x8 */
pub const BOARD: [[char; 8]; 8] = [
    ['_', '_', '_', '_', '_', '_', '_', '_'],
    ['_', '_', '_', '_', '_', '_', '_', '_'],
    ['_', '_', '_', '_', '_', '_', '_', '_'],
    ['_', '_', '_', '_', '_', '_', '_', '_'],
    ['_', '_', '_', '_', '_', '_', '_', '_'],
    ['_', '_', '_', '_', '_', '_', '_', '_'],
    ['_', '_', '_', '_', '_', '_', '_', '_'],
    ['_', '_', '_', '_', '_', '_', '_', '_'],
];

pub fn get_pieces() -> [Piece; 32] {
    [
        Piece::new(('a', 1), Team::White, Type::Pawn),
        Piece::new(('a', 6), Team::Black, Type::Pawn),
        Piece::new(('b', 1), Team::White, Type::Pawn),
        Piece::new(('b', 6), Team::Black, Type::Pawn),
        Piece::new(('c', 1), Team::White, Type::Pawn),
        Piece::new(('c', 6), Team::Black, Type::Pawn),
        Piece::new(('d', 1), Team::White, Type::Pawn),
        Piece::new(('d', 6), Team::Black, Type::Pawn),
        Piece::new(('e', 1), Team::White, Type::Pawn),
        Piece::new(('e', 6), Team::Black, Type::Pawn),
        Piece::new(('f', 1), Team::White, Type::Pawn),
        Piece::new(('f', 6), Team::Black, Type::Pawn),
        Piece::new(('g', 1), Team::White, Type::Pawn),
        Piece::new(('g', 6), Team::Black, Type::Pawn),
        Piece::new(('h', 1), Team::White, Type::Pawn),
        Piece::new(('h', 6), Team::Black, Type::Pawn),
        Piece::new(('c', 0), Team::White, Type::Bishop),
        Piece::new(('f', 0), Team::White, Type::Bishop),
        Piece::new(('c', 7), Team::Black, Type::Bishop),
        Piece::new(('f', 7), Team::Black, Type::Bishop),
        Piece::new(('b', 0), Team::White, Type::Knight),
        Piece::new(('g', 0), Team::White, Type::Knight),
        Piece::new(('b', 7), Team::Black, Type::Knight),
        Piece::new(('g', 7), Team::Black, Type::Knight),
        Piece::new(('a', 0), Team::White, Type::Rook),
        Piece::new(('h', 0), Team::White, Type::Rook),
        Piece::new(('a', 7), Team::Black, Type::Rook),
        Piece::new(('h', 7), Team::Black, Type::Rook),
        Piece::new(('e', 0), Team::White, Type::King),
        Piece::new(('e', 7), Team::Black, Type::King),
        Piece::new(('d', 0), Team::White, Type::Queen),
        Piece::new(('d', 7), Team::Black, Type::Queen),
    ]
}
