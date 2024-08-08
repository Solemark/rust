#[cfg(test)]
mod tests {
    use crate::gacha_roll::{roll, Game};

    #[test]
    fn test_game_string_appears() {
        let res = roll(Game::Fgo);
        assert_eq!(true, res.contains("FGO"));

        let res = roll(Game::Ak);
        assert_eq!(true, res.contains("AK"));

        let res = roll(Game::Gi);
        assert_eq!(true, res.contains("GI"));
    }
}
