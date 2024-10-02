#[cfg(test)]
mod test {
    use crate::{piece::Piece, tictactoe::TicTacToe};

    fn create_game(board: [Piece; 9]) -> TicTacToe {
        TicTacToe { board }
    }

    #[test]
    fn test_crosses_row() {
        let game = create_game([
            Piece::X,
            Piece::X,
            Piece::X,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
        ]);
        assert_eq!(game.check_board(), "Crosses wins");
    }
    #[test]
    fn test_noughts_row() {
        let game = create_game([
            Piece::O,
            Piece::O,
            Piece::O,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
        ]);
        assert_eq!(game.check_board(), "Noughts wins");
    }
    #[test]
    fn test_crosses_col() {
        let game = create_game([
            Piece::X,
            Piece::N,
            Piece::N,
            Piece::X,
            Piece::N,
            Piece::N,
            Piece::X,
            Piece::N,
            Piece::N,
        ]);
        assert_eq!(game.check_board(), "Crosses wins");
    }
    #[test]
    fn test_noughts_col() {
        let game = create_game([
            Piece::O,
            Piece::N,
            Piece::N,
            Piece::O,
            Piece::N,
            Piece::N,
            Piece::O,
            Piece::N,
            Piece::N,
        ]);
        assert_eq!(game.check_board(), "Noughts wins");
    }
    #[test]
    fn test_crosses_diag() {
        let game = create_game([
            Piece::X,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::X,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::X,
        ]);
        assert_eq!(game.check_board(), "Crosses wins");
    }
    #[test]
    fn test_noughts_diag() {
        let game = create_game([
            Piece::O,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::O,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::O,
        ]);

        assert_eq!(game.check_board(), "Noughts wins");
    }
    #[test]
    fn test_no_winner() {
        let game = create_game([
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
            Piece::N,
        ]);
        assert_eq!(game.check_board(), String::new());
    }
}
