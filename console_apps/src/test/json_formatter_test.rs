#[cfg(test)]
mod tests {
    use crate::json_formatter::str_to_json;

    #[test]
    fn test_str_to_json() {
        assert_eq!(
            String::from("{\"message\": \"Hello World!\"}"),
            str_to_json(String::from("message"), String::from("Hello World!"))
        );
    }
}
