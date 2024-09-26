mod test;
mod tictactoe;

use crate::tictactoe::TicTacToe;

fn main() {
    let board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    TicTacToe { board }.cli();
}
