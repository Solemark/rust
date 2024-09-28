use crate::{
    default::{get_pieces, BOARD},
    pieces::{Piece, Team},
};

pub struct Game {
    board: [[char; 8]; 8],
    pieces: [Piece; 32],
    turn: Team,
    player: Team,
}

impl Game {
    pub fn new() -> Game {
        let board = BOARD;
        let pieces = get_pieces();
        let turn = Team::White;
        let player = Team::White;
        Game {
            board,
            pieces,
            turn,
            player,
        }
    }

    pub fn play(&mut self) {
        for _ in 0..=2 {
            let turn = Into::<String>::into(self.turn.clone());
            println!("turn: {}\n{}\n{} to move", turn, self.draw_all(), turn);
            self.turn = match self.turn {
                Team::Black => Team::White,
                Team::White => Team::Black,
            };
            self.player = match self.player {
                Team::Black => Team::White,
                Team::White => Team::Black,
            };
        }
    }

    pub fn draw_all(&mut self) -> String {
        self.board = BOARD;
        self.draw_pieces();
        self.draw_board()
    }

    pub fn set_player(&mut self, player: Team) -> &mut Self {
        self.player = player;
        self
    }

    fn draw_pieces(&mut self) {
        for piece in &self.pieces {
            let pos = piece.get_position();
            let char = match self.char_to_usize(pos.0) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            self.board[pos.1][char] = piece.get_piece().into();
        }
    }

    fn draw_board(&self) -> String {
        let mut rows: [String; 9] = [const { String::new() }; 9];
        let mut i: usize = 0;
        for r in self.board {
            let mut row = (i + 1).to_string();
            for c in r {
                row += &format!("|{}|", c);
            }
            rows[i as usize] = row;
            i += 1;
        }
        if self.player == Team::White {
            self.flip_board(&mut rows);
        }
        rows[8] = String::from("  A  B  C  D  E  F  G  H");
        rows.join("\n")
    }

    fn flip_board(&self, rows: &mut [String; 9]) {
        let (mut i, mut j) = (0, 7);
        while i < j {
            let (x, y) = (rows[i].clone(), rows[j].clone());
            (rows[i], rows[j]) = (y, x);
            (i, j) = (i + 1, j - 1);
        }
    }

    fn char_to_usize(&self, pos: char) -> Result<usize, String> {
        match pos {
            'a' => Ok(0),
            'b' => Ok(1),
            'c' => Ok(2),
            'd' => Ok(3),
            'e' => Ok(4),
            'f' => Ok(5),
            'g' => Ok(6),
            'h' => Ok(7),
            _ => Err(format!("Char: {}, is invalid", pos)),
        }
    }
}
