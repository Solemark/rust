mod consts;
mod piece;
mod test;
mod tictactoe;
use crate::tictactoe::TicTacToe;

fn main() {
    TicTacToe::new().cli()
}
