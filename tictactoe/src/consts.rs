use crate::piece::Piece;

/**all the potential win positions on the board*/
pub const ALL_CHECK: [[usize; 3]; 8] = [
    //rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    //columns
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    //diagonals
    [0, 4, 8],
    [2, 4, 6],
];
pub const GAME_KEY: &str = "0 | 1 | 2\n3 | 4 | 5\n6 | 7 | 8";
pub const NEW_BOARD: [Piece; 9] = [
    Piece::N,
    Piece::N,
    Piece::N,
    Piece::N,
    Piece::N,
    Piece::N,
    Piece::N,
    Piece::N,
    Piece::N,
];
