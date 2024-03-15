use std::io::{stdin, stdout, Write};
pub fn w2() -> String {
    let max_marks: f32 = 65.0;
    let name: String = get_name();
    let mark: f32 = get_mark(&name, &max_marks);

    format!(
        "Student recieved {}% of total marks",
        ((mark * 100.0) / max_marks)
    )
}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush!");
    stdin().read_line(input).expect("Failed to read!");
}

fn get_name() -> String {
    let mut input = String::new();
    print!("Enter the name of the student: ");
    read(&mut input);
    match input.trim_end() {
        name => name.to_string(),
    }
}

fn get_mark(name: &String, max_marks: &f32) -> f32 {
    let mut input = String::new();
    print!("Enter the mark of {} out of {}: ", name, &max_marks);
    read(&mut input);
    match input.trim_end() {
        mark => mark.parse().expect("Error! Unable to parse as number"),
    }
}
