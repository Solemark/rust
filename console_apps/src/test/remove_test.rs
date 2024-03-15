#[cfg(test)]
mod tests {
    use crate::remove::chars;

    #[test]
    fn test_hello_world_to_heoword() {
        let input: String = String::from("Hello World!");
        let cs: Vec<char> = vec![' ', 'l'];
        let expect: String = String::from("HeoWord!");
        assert_eq!(expect, chars(input, cs));
    }
}
