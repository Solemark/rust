#[cfg(test)]
mod tests {
    use crate::dice_roll::roll;

    #[test]
    fn test_dice_roll_d6() {
        let expect: [u32; 6] = [1, 2, 3, 4, 5, 6];
        let result: u32 = roll(6);
        assert!(expect.contains(&result));
    }

    #[test]
    fn test_dice_roll_d8() {
        let expect: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let result: u32 = roll(8);
        assert!(expect.contains(&result));
    }

    #[test]
    fn test_dice_roll_d10() {
        let expect: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: u32 = roll(10);
        assert!(expect.contains(&result));
    }

    #[test]
    fn test_dice_roll_d12() {
        let expect: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let result: u32 = roll(12);
        assert!(expect.contains(&result));
    }

    #[test]
    fn test_dice_roll_d20() {
        let expect: [u32; 20] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let result: u32 = roll(20);
        assert!(expect.contains(&result));
    }
}
