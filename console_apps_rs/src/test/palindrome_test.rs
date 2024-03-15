#[cfg(test)]
mod tests {
    use crate::palindrome::palindrome;

    #[test]
    fn test_check_palindrome() {
        assert_eq!(palindrome(String::from("ABCDCBA"), None, None), true);
    }
    #[test]
    fn test_check_case_sensitivity() {
        assert_eq!(palindrome(String::from("ABCDcba"), None, None), false);
    }
}
