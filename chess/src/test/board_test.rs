#[cfg(test)]
mod test {
    use crate::{game::Game, pieces::Team};

    #[test]
    fn test_draw_new_board_white() {
        let exp = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "8|R||H||B||Q||K||B||H||R|",
            "7|P||P||P||P||P||P||P||P|",
            "6|_||_||_||_||_||_||_||_|",
            "5|_||_||_||_||_||_||_||_|",
            "4|_||_||_||_||_||_||_||_|",
            "3|_||_||_||_||_||_||_||_|",
            "2|P||P||P||P||P||P||P||P|",
            "1|R||H||B||Q||K||B||H||R|",
            "  A  B  C  D  E  F  G  H",
        );
        let res = Game::new().draw_all();
        assert_eq!(exp, res)
    }

    #[test]
    fn test_draw_new_board_black() {
        let exp = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "1|R||H||B||Q||K||B||H||R|",
            "2|P||P||P||P||P||P||P||P|",
            "3|_||_||_||_||_||_||_||_|",
            "4|_||_||_||_||_||_||_||_|",
            "5|_||_||_||_||_||_||_||_|",
            "6|_||_||_||_||_||_||_||_|",
            "7|P||P||P||P||P||P||P||P|",
            "8|R||H||B||Q||K||B||H||R|",
            "  A  B  C  D  E  F  G  H",
        );
        let res = Game::new().set_player(Team::Black).draw_all();
        assert_eq!(exp, res)
    }
}
