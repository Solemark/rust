#[cfg(test)]
mod tests {
    use crate::filter::{evens, negative, odds, positive};
    fn get_data() -> Vec<i32> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    }

    fn get_data_negative() -> Vec<i32> {
        vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]
    }

    #[test]
    fn test_filter_negative() {
        let input: Vec<i32> = get_data_negative();
        let expect: Vec<i32> = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = negative(input);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_filter_positive() {
        let input: Vec<i32> = get_data_negative();
        let expect: Vec<i32> = vec![-5, -4, -3, -2, -1];
        let result: Vec<i32> = positive(input);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_find_evens() {
        let answer: Vec<i32> = vec![1, 3, 5, 7, 9];
        assert_eq!(evens(get_data()), answer);
    }

    #[test]
    fn test_find_evens_negative() {
        let answer: Vec<i32> = vec![-5, -3, -1, 1, 3, 5];
        assert_eq!(evens(get_data_negative()), answer);
    }

    #[test]
    fn test_find_odds() {
        let answer: Vec<i32> = vec![2, 4, 6, 8, 10];
        assert_eq!(odds(get_data()), answer);
    }

    #[test]
    fn test_filter_odds_negative() {
        let answer: Vec<i32> = vec![-4, -2, 0, 2, 4];
        assert_eq!(odds(get_data_negative()), answer);
    }
}
