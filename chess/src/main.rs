use game::Game;
mod default;
mod game;
mod pieces;
mod test;

fn main() {
    Game::new().play();
}
