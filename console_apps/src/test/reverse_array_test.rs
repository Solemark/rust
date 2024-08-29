#[cfg(test)]
mod tests {
    use crate::reverse_array::same_arr;

    #[test]
    fn test_reverse_samearray() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expect = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let result = same_arr(input);
        assert_eq!(expect, result)
    }
}
