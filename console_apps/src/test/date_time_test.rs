#[cfg(test)]
mod tests {
    use crate::date_time::check_time;

    #[test]
    fn test_current_date() {
        assert_eq!(check_time(), "the date is Sunday the 22nd of October 2023");
    }
}
