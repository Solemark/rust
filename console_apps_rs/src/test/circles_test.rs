#[cfg(test)]
mod tests {
    use crate::circles::{area, perimeter};
    use std::f32::consts::PI;

    fn get_data() -> [f32; 4] {
        [0.0, 5.0, -5.0, -0.0]
    }

    #[test]
    fn test_circle_area() {
        let input: [f32; 4] = get_data();
        for r in input {
            let mut expect: f32 = 0.0;
            if r > 0.0 {
                expect = PI * (r * r);
            }
            let result: f32 = area(r);
            assert_eq!(expect, result);
        }
    }

    #[test]
    fn test_circle_perim() {
        let input: [f32; 4] = get_data();
        for r in input {
            let mut expect: f32 = 0.0;
            if r > 0.0 {
                expect = 2.0 * PI * r;
            }
            let result: f32 = perimeter(r);
            assert_eq!(expect, result);
        }
    }
}
