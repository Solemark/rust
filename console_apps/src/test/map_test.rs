#[cfg(test)]
mod tests {
    use crate::map::map;

    fn double(x: i32) -> i32 {
        x * 2
    }
    fn doublef(x: f32) -> f32 {
        x * 2.0
    }

    #[test]
    fn test_map_double() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expect: Vec<i32> = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        let result: Vec<i32> = map(input, double);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_map_double2() {
        let input: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let expect: Vec<f32> = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0];
        let result: Vec<f32> = map(input, doublef);
        assert_eq!(expect, result);
    }
}
