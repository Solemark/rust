#[cfg(test)]
mod test {
    use crate::tictactoe::TicTacToe;

    fn create_game(board: [i8; 9]) -> TicTacToe {
        TicTacToe { board }
    }

    #[test]
    fn test_crosses_row() {
        let game = create_game([1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(game.check_board().unwrap(), "Crosses wins");
    }
    #[test]
    fn test_noughts_row() {
        let game = create_game([-1, -1, -1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(game.check_board().unwrap(), "Noughts wins");
    }
    #[test]
    fn test_crosses_col() {
        let game = create_game([1, 0, 0, 1, 0, 0, 1, 0, 0]);
        assert_eq!(game.check_board().unwrap(), "Crosses wins");
    }
    #[test]
    fn test_noughts_col() {
        let game = create_game([-1, 0, 0, -1, 0, 0, -1, 0, 0]);
        assert_eq!(game.check_board().unwrap(), "Noughts wins");
    }
    #[test]
    fn test_crosses_diag() {
        let game = create_game([1, 0, 0, 0, 1, 0, 0, 0, 1]);
        assert_eq!(game.check_board().unwrap(), "Crosses wins");
    }
    #[test]
    fn test_noughts_diag() {
        let game = create_game([-1, 0, 0, 0, -1, 0, 0, 0, -1]);

        assert_eq!(game.check_board().unwrap(), "Noughts wins");
    }
    #[test]
    fn test_no_winner() {
        let game = create_game([0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(game.check_board(), None);
    }
}
