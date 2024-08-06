#[allow(dead_code)]
pub fn palindrome(input: String) -> bool {
    run(input.clone(), 0, input.len() - 1)
}

fn run(input: String, x: usize, y: usize) -> bool {
    if x > y {
        return true;
    }
    if input.chars().nth(x + 1) != input.chars().nth(y - 1) {
        return false;
    }
    run(input, x + 1, y - 1)
}
