use rand::prelude::*;

#[allow(dead_code)]
pub enum Game {
    FGO,
    AK,
    GI,
}

#[allow(dead_code)]
pub fn roll(game: Game) -> String {
    let output: String;
    match game {
        Game::FGO => output = play(String::from("FGO"), 100, 300, 5),
        Game::AK => output = play(String::from("AK"), 50, 100, 6),
        Game::GI => output = play(String::from("GI"), 60, 90, 5),
    }
    output
}

fn play(game: String, rate: i32, pity: i32, rarity: i32) -> String {
    for roll in 1..=pity {
        if rate == rand::thread_rng().gen_range(1..=rate) {
            return format!("It took {}/{} to get a {}* in {}", roll, pity, rarity, game);
        }
    }
    format!("You hit pity at {} for a {}* in {}", pity, rarity, game)
}
