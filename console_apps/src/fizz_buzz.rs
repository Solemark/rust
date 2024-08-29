#[allow(dead_code)]
pub fn fizz_buzz(fizz: u32, buzz: u32, max: u32) -> String {
    let mut output = String::new();
    for i in 1..=max {
        output += if_then(i % fizz == 0, "fizz");
        output += if_then(i % buzz == 0, "buzz");
        output += if_then(!output.ends_with('z'), &i.to_string());
        output += if_then(i != max, ", ");
    }
    output
}

fn if_then(condition: bool, success: &str) -> &str {
    if condition {
        return success;
    }
    ""
}
