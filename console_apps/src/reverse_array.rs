#[allow(dead_code)]
pub fn same_arr<N: Clone>(mut input: Vec<N>) -> Vec<N> {
    let mut i = 0;
    let mut j = input.len() - 1;
    while i <= j {
        let x = input[i].clone();
        let y = input[j].clone();
        input[j] = x;
        input[i] = y;
        i += 1;
        j -= 1;
    }
    input
}
