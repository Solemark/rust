pub fn draw_board(field: [[char; 10]; 10]) -> String {
    let mut out: String = String::new();
    for r in field {
        for c in r {
            out.push(c);
        }
        out.push('\n');
    }
    out
}

fn if_then_else(c: bool, a: usize, b: usize) -> usize {
    if c {
        a
    } else {
        b
    }
}

pub fn calculate(mut field: [[char; 10]; 10], mut row: usize, mut col: usize) -> [[char; 10]; 10] {
    let allowed: [char; 3] = [' ', 'T', 'X'];
    let mut diversions: Vec<[usize; 2]> = vec![];
    loop {
        let mut moved: bool = false;
        let t: usize = if_then_else((col - 1) > 0, col - 1, 0);
        let l: usize = if_then_else((row - 1) > 0, row - 1, 0);
        let r: usize = if_then_else((row + 1) < (field.len() - 1), row + 1, field.len() - 1);
        let b: usize = if_then_else((col + 1) < (field.len() - 1), col + 1, field.len() - 1);
        let l2: char = field[l][col];
        let r2: char = field[r][col];
        let t2: char = field[row][t];
        let b2: char = field[row][b];
        if allowed.contains(&l2) && !moved {
            if l2 == 'T' {
                diversions.push([l, col]);
            }
            row -= 1;
            moved = true;
        }
        if allowed.contains(&r2) && !moved {
            if r2 == 'T' {
                diversions.push([r, col]);
            }
            row += 1;
            moved = true;
        }
        if allowed.contains(&t2) && !moved {
            if t2 == 'T' {
                diversions.push([row, t]);
            }
            col -= 1;
            moved = true;
        }
        if allowed.contains(&b2) && !moved {
            if t2 == 'T' {
                diversions.push([row, b]);
            }
            col += 1;
            moved = true;
        }
        field[row][col] = '+';

        if l2 == 'X' || r2 == 'X' || t2 == 'X' || b2 == 'X' {
            break;
        }

        if !moved {
            row = diversions[0][0];
            col = diversions[0][1];
            diversions.remove(0);
        }
    }
    field
}
