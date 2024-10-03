#[cfg(test)]
mod tests {
    use crate::reduce::{add, div, mul, other, sub};

    #[test]
    fn test_add() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = add(input);
        assert_eq!(55, result);
    }

    #[test]
    fn test_sub() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = sub(input);
        assert_eq!(-53, result);
    }

    #[test]
    fn test_mul() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = mul(input);
        assert_eq!(3628800, result);
    }

    #[test]
    fn test_div() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = div(input);
        assert_eq!(0, result);
    }

    #[test]
    fn test_other() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: i32 = other(input, other_reduce);
        assert_eq!(100, result);
    }

    fn other_reduce(a: i32, b: i32) -> i32 {
        (a - 1) + (b * 2)
    }
}
