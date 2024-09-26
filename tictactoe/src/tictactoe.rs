use std::io::{stdin, stdout, Write};

enum Winner {
    Cross,
    Nought,
    None,
}

/**all the potential win positions on the board*/
const ALL_CHECK: [[usize; 3]; 8] = [
    //rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    //columns
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    //diagonals
    [0, 4, 8],
    [2, 4, 6],
];

pub struct TicTacToe {
    pub board: [i8; 9],
}

impl TicTacToe {
    /**run the game as a cli*/
    pub fn cli(&mut self) {
        let game_key: &str = "0 | 1 | 2\n3 | 4 | 5\n6 | 7 | 8";
        let mut turn = 1;
        loop {
            if turn > 9 {
                println!("{}\n{}", self.draw_board(), "No Winner!");
                break;
            }
            let pos = self.get_move(&game_key);
            let val: i8 = {
                let mut out = 1;
                if turn % 2 == 0 {
                    out = -1
                }
                out
            };
            self.board[pos] = val;
            match self.check_board() {
                Some(res) => {
                    println!("{}\n{}", self.draw_board(), res);
                    break;
                }
                None => turn += 1,
            }
        }
    }

    /**make sure the input move is valid*/
    fn get_move(&self, game_key: &str) -> usize {
        loop {
            println!("{}", self.draw_board());
            println!("Enter the position of the next move\n{}", game_key);

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
        let mut output: String = String::new();
        let mut c: i8 = 0;
        for i in self.board {
            match i {
                1 => output.push_str("|X|"),
                -1 => output.push_str("|0|"),
                0 => output.push_str("|_|"),
                _ => println!("Unknown board type!: {}", i),
            }
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
            if self.board[pos] != 0 {
                return false;
            }
            true
        }
    }

    /**check all possible "win states" on the board*/
    pub fn check_board(&self) -> Option<String> {
        let mut res: Option<String> = None;
        for check in ALL_CHECK {
            match self.check_cells(
                self.board[check[0]],
                self.board[check[1]],
                self.board[check[2]],
            ) {
                Winner::Cross => {
                    res = Some(String::from("Crosses wins"));
                    break;
                }
                Winner::Nought => {
                    res = Some(String::from("Noughts wins"));
                    break;
                }
                Winner::None => {
                    res = None;
                }
            };
        }
        res
    }

    /**check if the provided board positions result in a win*/
    fn check_cells(&self, a: i8, b: i8, c: i8) -> Winner {
        match a + b + c {
            3 => Winner::Cross,
            -3 => Winner::Nought,
            _ => Winner::None,
        }
    }
}
