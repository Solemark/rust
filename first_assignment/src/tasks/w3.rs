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
    let output: String;
    match value {
        0..=49 => output = String::from("Failed"),
        50..=64 => output = String::from("Passed"),
        65..=74 => output = String::from("Credit"),
        75..=84 => output = String::from("Distinction"),
        85..=100 => output = String::from("High Distinction"),
        _ => output = format!("Error! Number {} out of bounds!", value),
    }
    output
}

pub fn w3() -> String {
    let mut input = String::new();
    print!("Enter student mark (0-100): ");
    read(&mut input);
    get_grade(input)
}
