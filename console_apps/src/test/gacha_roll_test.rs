#[cfg(test)]
mod tests {
    use crate::gacha_roll::{GachaRoll, Game};

    #[test]
    fn test_game_string_appears() {
        let res = GachaRoll::new(Game::Fgo).play();
        assert_eq!(true, res.contains("FGO"));

        let res = GachaRoll::new(Game::Ak).play();
        assert_eq!(true, res.contains("AK"));

        let res = GachaRoll::new(Game::Gi).play();
        assert_eq!(true, res.contains("GI"));
    }
}
