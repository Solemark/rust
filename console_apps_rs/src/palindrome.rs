#[allow(dead_code)]
pub fn palindrome(input: String, x: Option<usize>, y: Option<usize>) -> bool {
    if x > y {
        return true;
    }
    let i = x.unwrap_or_default();
    let j = y.unwrap_or(input.len() - 1);
    if input.chars().nth(i) != input.chars().nth(j) {
        return false;
    }
    palindrome(input, Some(i + 1), Some(j - 1))
}
