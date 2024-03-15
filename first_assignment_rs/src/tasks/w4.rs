use std::io::{stdin, stdout, Write};
struct Week4 {
    name: String,
    mark: i32,
}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush!");
    stdin().read_line(input).expect("Failed to read!");
}

fn get_name() -> String {
    let mut input: String = String::new();
    print!("Enter student name: ");
    read(&mut input);
    input
}

fn get_mark() -> i32 {
    let mut input: String = String::new();
    print!("Enter student mark: ");
    read(&mut input);
    input
        .trim()
        .parse()
        .expect("Unable to parse string to number!")
}

fn get_grade(input: i32) -> String {
    let output: String;
    match input {
        0..=49 => output = String::from("Failed"),
        50..=64 => output = String::from("Passed"),
        65..=74 => output = String::from("Credit"),
        75..=84 => output = String::from("Distinction"),
        85..=100 => output = String::from("High Distinction"),
        _ => output = format!("Error! Number {} out of bounds!", input),
    }
    output
}

pub fn w4() -> String {
    println!("\t\tMark Entry System");
    let max = 3;
    let mut total: i32 = 0;
    for _ in 0..max {
        let student = Week4 {
            name: get_name(),
            mark: get_mark(),
        };
        total += &student.mark;
        println!(
            "{} with mark: {} received a grade of: {}",
            &student.name,
            &student.mark,
            get_grade(student.mark)
        );
    }
    format!("The average mark is {}", (total / max))
}
