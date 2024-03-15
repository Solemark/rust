#[allow(dead_code)]
pub fn play(fizz: i32, buzz: i32, max: i32, i: Option<i32>, output: Option<String>) -> String {
    let j: i32 = i.unwrap_or(1);
    let mut str: String = output.unwrap_or_default();
    if j > max {
        return str;
    }
    str += if_then(j % fizz == 0, "fizz");
    str += if_then(j % buzz == 0, "buzz");
    str += if_then(!str.ends_with('z'), &j.to_string());
    str += if_then(j != max, ", ");
    play(fizz, buzz, max, Some(j + 1), Some(str))
}

fn if_then(condition: bool, success: &str) -> &str {
    if condition {
        return success;
    }
    ""
}
