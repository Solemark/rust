use rand::prelude::*;

#[allow(dead_code)]
pub enum Game {
    Fgo,
    Ak,
    Gi,
}

#[allow(dead_code)]
pub fn roll(game: Game) -> String {
    match game {
        Game::Fgo => play("FGO", 100, 300, 5),
        Game::Ak => play("AK", 50, 100, 6),
        Game::Gi => play("GI", 60, 90, 5),
    }
}

fn play(game: &str, rate: i32, pity: i32, rare: i32) -> String {
    for roll in 1..=pity {
        if rate == rand::thread_rng().gen_range(1..=rate) {
            return format!("It took {}/{} to get a {}* in {}", roll, pity, rare, game);
        }
    }
    format!("You hit pity at {} for a {}* in {}", pity, rare, game)
}
