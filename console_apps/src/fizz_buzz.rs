#[allow(dead_code)]
pub fn fizz_buzz(fizz: i32, buzz: i32, max: i32) -> String {
    play(fizz, buzz, max, 1, String::new())
}
fn play(fizz: i32, buzz: i32, max: i32, i: i32, output: String) -> String {
    if i > max {
        return output;
    }
    let mut str = String::new();
    str += if_then(i % fizz == 0, "fizz");
    str += if_then(i % buzz == 0, "buzz");
    str += if_then(!str.ends_with('z'), &i.to_string());
    str += if_then(i != max, ", ");
    play(fizz, buzz, max, i + 1, output + &str)
}

fn if_then(condition: bool, success: &str) -> &str {
    if condition {
        return success;
    }
    ""
}
