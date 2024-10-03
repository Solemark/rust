use rand::prelude::*;

#[allow(dead_code)]
pub enum Game {
    Fgo,
    Ak,
    Gi,
}
impl Game {
    fn to_string(&self) -> String {
        match self {
            Game::Fgo => String::from("FGO"),
            Game::Ak => String::from("AK"),
            Game::Gi => String::from("GI"),
        }
    }
}

pub struct GachaRoll {
    game: Game,
    rate: u32,
    pity: u32,
    rare: u32,
}
#[allow(dead_code)]
impl GachaRoll {
    pub fn new(game: Game) -> GachaRoll {
        match game {
            Game::Fgo => GachaRoll {
                game,
                rate: 100,
                pity: 300,
                rare: 5,
            },
            Game::Ak => GachaRoll {
                game,
                rate: 50,
                pity: 100,
                rare: 6,
            },
            Game::Gi => GachaRoll {
                game,
                rate: 60,
                pity: 90,
                rare: 5,
            },
        }
    }

    pub fn play(&self) -> String {
        for roll in 1..self.pity {
            if self.rate == rand::thread_rng().gen_range(1..=self.rate) {
                return self.get_win_string(roll);
            }
        }
        self.get_pity_string()
    }

    fn get_win_string(&self, roll: u32) -> String {
        format!(
            "It took {}/{} to get a {}* in {}",
            roll,
            self.pity,
            self.rare,
            self.game.to_string()
        )
    }

    fn get_pity_string(&self) -> String {
        format!(
            "You hit pity at {} for a {}* in {}",
            self.pity,
            self.rare,
            self.game.to_string()
        )
    }
}
