#[allow(dead_code)]
pub fn same_arr<N>(mut input: Vec<N>) -> Vec<N>
where
    N: Copy,
{
    let mut x: N;
    let mut y: N;
    let mut i: usize = 0;
    let mut c: usize = input.len() - 1;

    while i <= c {
        x = input[i];
        y = input[c];
        input[c] = x;
        input[i] = y;
        i += 1;
        c -= 1;
    }
    input
}
