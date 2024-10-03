#[allow(dead_code)]
pub fn same_array<N: Copy>(mut arr: Vec<N>) -> Vec<N> {
    let (mut i, mut j) = (0, arr.len() - 1);
    while i <= j {
        let (x, y) = (arr[i], arr[j]);
        (arr[i], arr[j]) = (y, x);
        (i, j) = (i + 1, j - 1);
    }
    arr
}
