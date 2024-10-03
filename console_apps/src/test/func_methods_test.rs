#[cfg(test)]
mod test {
    use crate::func_methods::{iter_map, iter_reduce};

    fn get_vec() -> Vec<i8> {
        vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    }

    fn doubling<N: std::ops::Add<Output = N> + Clone>(num: N) -> N {
        num.clone() + num
    }

    fn reducing<N: std::ops::Add<Output = N>>(t: N, i: N) -> N {
        t + i
    }

    #[test]
    fn test_iter_map() {
        let exp = vec![-10, -8, -6, -4, -2, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        let res = iter_map(get_vec(), &doubling);
        assert_eq!(exp, res)
    }

    #[test]
    fn test_iter_reduce() {
        let exp = 40;
        let res = iter_reduce(get_vec(), reducing);
        assert_eq!(exp, res)
    }
}
