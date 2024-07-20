use std::io::{stdin, stdout, Write};
fn read(input: &mut String) {
    stdout().flush().expect("failed to flush!");
    stdin().read_line(input).expect("Failed to read!");
}

fn get_grade(input: String) -> String {
    let value: i8 = input
        .trim()
        .parse()
        .expect("Failed to pass String to number");
    match value {
        0..=49 => String::from("Failed"),
        50..=64 => String::from("Passed"),
        65..=74 => String::from("Credit"),
        75..=84 => String::from("Distinction"),
        85..=100 => String::from("High Distinction"),
        _ => format!("Error! Number {} out of bounds!", value),
    }
}

pub fn w3() -> String {
    let mut input = String::new();
    print!("Enter student mark (0-100): ");
    read(&mut input);
    get_grade(input)
}
