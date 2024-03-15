use std::io::{stdin, stdout, Write};
fn check_cells(a: i8, b: i8, c: i8) -> &'static str {
    match a + b + c {
        3 => "crosses wins",
        -3 => "noughts wins",
        _ => "",
    }
}

pub fn check_board(arr: &[i8; 9]) -> String {
    let mut output: String = String::new();
    //check rows
    output.push_str(check_cells(arr[0], arr[1], arr[2]));
    output.push_str(check_cells(arr[3], arr[4], arr[5]));
    output.push_str(check_cells(arr[6], arr[7], arr[8]));
    // check columns
    output.push_str(check_cells(arr[0], arr[3], arr[6]));
    output.push_str(check_cells(arr[1], arr[4], arr[7]));
    output.push_str(check_cells(arr[2], arr[5], arr[8]));
    //check diagonals
    output.push_str(check_cells(arr[0], arr[4], arr[8]));
    output.push_str(check_cells(arr[2], arr[4], arr[6]));
    output
}

fn draw_board(arr: &[i8; 9]) -> String {
    let mut output: String = String::new();
    let mut c: i8 = 0;
    for i in arr {
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

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush!");
    stdin().read_line(input).expect("Failed to read!");
}

pub fn cli() {
    let mut arr: [i8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let game_key: &str = "0 | 1 | 2\n3 | 4 | 5\n6 | 7 | 8";
    let mut flag: bool = true;
    let mut result: String;

    loop {
        let mut pos: String = String::new();
        println!("{}", draw_board(&arr));
        println!("Enter the position of the next move\n{}", game_key);
        read(&mut pos);
        let pos: usize = pos.trim().parse().unwrap();
        let mut val: i8 = 1;
        if !flag {
            val = -1;
        }
        arr[pos] = val;
        flag = !flag;
        result = check_board(&arr);
        if !result.is_empty() {
            println!("{}\n{}", draw_board(&arr), result);
            break;
        }
    }
}
