#[cfg(test)]
mod tests {
    use crate::reduce::{red_add, red_div, red_mul, red_sub};

    #[test]
    fn test_red_add() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = red_add(input);
        assert_eq!(55, result);
    }

    #[test]
    fn test_red_sub() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = red_sub(input);
        assert_eq!(-53, result);
    }

    #[test]
    fn test_red_mul() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = red_mul(input);
        assert_eq!(3628800, result);
    }

    #[test]
    fn test_red_div() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = red_div(input);
        assert_eq!(0, result);
    }
}
