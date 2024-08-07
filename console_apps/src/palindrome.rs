#[allow(dead_code)]
pub fn palindrome(input: String) -> bool {
    for i in 0..input.len() {
        if input.chars().nth(i) != input.chars().nth(input.len() - (i + 1)) {
            return false;
        }
    }
    true
}
