#[cfg(test)]
mod test {
    use crate::multiplication_tables::get_table;

    #[test]
    fn test_multiplication_tables() {
        let expect: Vec<i32> = vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];
        let result: Vec<i32> = get_table(2, 12);
        assert_eq!(expect, result);
    }
}
