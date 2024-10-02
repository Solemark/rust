use crate::{
    consts::{ALL_CHECK, GAME_KEY, NEW_BOARD},
    piece::Piece,
};
use std::io::{stdin, stdout, Write};

pub struct TicTacToe {
    pub board: [Piece; 9],
}

impl TicTacToe {
    /** Create a new game board */
    pub fn new() -> TicTacToe {
        TicTacToe { board: NEW_BOARD }
    }

    /**run the game as a cli*/
    pub fn cli(&mut self) {
        let mut turn = 1;
        loop {
            if turn > 9 {
                println!("{}\nNo Winner!", self.draw_board());
                break;
            }

            if turn % 2 == 0 {
                self.board[self.get_move()] = Piece::O;
            } else {
                self.board[self.get_move()] = Piece::X;
            }

            let res = self.check_board();
            if res != String::new() {
                println!("{}\n{}", self.draw_board(), res);
                break;
            } else {
                turn += 1;
            }
        }
    }

    /**make sure the input move is valid*/
    fn get_move(&self) -> usize {
        loop {
            println!(
                "{}\nEnter the position of the next move\n{}",
                self.draw_board(),
                GAME_KEY
            );

            let mut pos: String = String::new();
            self.read(&mut pos);
            let pos: usize = pos.trim().parse().unwrap_or(9);

            if self.check_legal_move(pos) {
                return pos;
            }
            println!("{} is not a valid move", pos);
        }
    }

    /**draw the board*/
    fn draw_board(&self) -> String {
        let (mut output, mut c) = (String::new(), 0);
        for m in &self.board {
            output.push_str(m.get_icon());
            c += 1;
            if c % 3 == 0 {
                c = 0;
                output.push('\n');
            }
        }
        output
    }

    /**flush, pass in and read from buffer*/
    fn read(&self, input: &mut String) {
        stdout().flush().expect("failed to flush!");
        stdin().read_line(input).expect("Failed to read!");
    }

    /**Check if input is a board position and if space is available*/
    fn check_legal_move(&self, pos: usize) -> bool {
        {
            if pos > 8 {
                return false;
            }
            if self.board[pos] != Piece::N {
                return false;
            }
            true
        }
    }

    /**check all possible "win states" on the board*/
    pub fn check_board(&self) -> String {
        for i in ALL_CHECK {
            let val = self.check_cells(&self.board[i[0]], &self.board[i[1]], &self.board[i[2]]);
            if val != Piece::N {
                return format!("{} wins", val.get_name());
            }
        }
        String::new()
    }

    /**check if the provided board positions result in a win*/
    fn check_cells(&self, a: &Piece, b: &Piece, c: &Piece) -> Piece {
        match a.get_value() + b.get_value() + c.get_value() {
            3 => Piece::X,
            -3 => Piece::O,
            _ => Piece::N,
        }
    }
}
