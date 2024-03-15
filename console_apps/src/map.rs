#[allow(dead_code)]
pub fn map<N>(input: Vec<N>, func: fn(N) -> N) -> Vec<N> {
    input.into_iter().map(func).collect::<Vec<N>>()
}
