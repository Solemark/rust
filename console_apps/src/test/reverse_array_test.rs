#[cfg(test)]
mod tests {
    use crate::reverse_array::same_array;

    #[test]
    fn test_reverse_samearray() {
        let inp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let exp = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let res = same_array(inp);
        assert_eq!(exp, res)
    }
}
