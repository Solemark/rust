pub fn draw_board(field: [[char; 10]; 10]) -> String {
    let mut output: String = String::new();
    for row in field {
        for cell in row {
            output.push(cell);
        }
        output.push('\n');
    }
    output
}

pub fn calculate(field: &mut [[char; 10]; 10], mut row: usize, mut col: usize) {
    let allowed_fields: [char; 3] = [' ', 'T', 'X'];
    let mut diversions: Vec<[usize; 2]> = vec![];
    loop {
        let row_1: usize = {
            let mut output: usize = 0;
            if (row - 1) > 0 {
                output = row - 1;
            }
            output
        };
        let row_2: usize = {
            let mut output: usize = field.len() - 1;
            if (row + 1) < (field.len() - 1) {
                output = row + 1;
            }
            output
        };
        let col_1: usize = {
            let mut output: usize = 0;
            if (col - 1) > 0 {
                output = col - 1;
            }
            output
        };
        let col_2: usize = {
            let mut output: usize = field.len() - 1;
            if (col + 1) < (field.len() - 1) {
                output = col + 1;
            }
            output
        };
        let mut moved: bool = false;
        let x1: char = field[row_1][col];
        let x2: char = field[row_2][col];
        let y1: char = field[row][col_1];
        let y2: char = field[row][col_2];
        if allowed_fields.contains(&x1) && !moved {
            if x1 == 'T' {
                diversions.push([row_1, col]);
            }
            row -= 1;
            moved = true;
        }
        if allowed_fields.contains(&x2) && !moved {
            if x2 == 'T' {
                diversions.push([row_2, col]);
            }
            row += 1;
            moved = true;
        }
        if allowed_fields.contains(&y1) && !moved {
            if y1 == 'T' {
                diversions.push([row, col_1]);
            }
            col -= 1;
            moved = true;
        }
        if allowed_fields.contains(&y2) && !moved {
            if y1 == 'T' {
                diversions.push([row, col_2]);
            }
            col += 1;
            moved = true;
        }
        field[row][col] = '+';

        if x1 == 'X' || x2 == 'X' || y1 == 'X' || y2 == 'X' {
            break;
        }

        if !moved {
            row = diversions[0][0];
            col = diversions[0][1];
            diversions.remove(0);
        }
    }
}
