#[cfg(test)]
mod tests {
    use crate::calculator::{add, div, mul, sub};

    #[test]
    fn test_positive_addition() {
        let expect: f32 = 3.0;
        let result: f32 = add(1.0, 2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_negative_addition() {
        let expect: f32 = -3.0;
        let result: f32 = add(-1.0, -2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_mixed_addition() {
        let expect: f32 = 1.0;
        let result: f32 = add(-1.0, 2.0);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_positive_subtraction() {
        let expect: f32 = -1.0;
        let result: f32 = sub(1.0, 2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_negative_subtraction() {
        let expect: f32 = 1.0;
        let result: f32 = sub(-1.0, -2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_mixed_subtraction() {
        let expect: f32 = -3.0;
        let result: f32 = sub(-1.0, 2.0);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_positive_multiplication() {
        let expect: f32 = 2.0;
        let result: f32 = mul(1.0, 2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_negative_multiplication() {
        let expect: f32 = 2.0;
        let result: f32 = mul(-1.0, -2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_mixed_multiplication() {
        let expect: f32 = -2.0;
        let result: f32 = mul(-1.0, 2.0);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_positive_division() {
        let expect: f32 = 0.5;
        let result: f32 = div(1.0, 2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_negative_division() {
        let expect: f32 = 0.5;
        let result: f32 = div(-1.0, -2.0);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_mixed_division() {
        let expect: f32 = -0.5;
        let result: f32 = div(-1.0, 2.0);
        assert_eq!(expect, result);
    }
}
