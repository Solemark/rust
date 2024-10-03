#[allow(dead_code)]
pub fn iter_map<N, F>(arr: Vec<N>, func: F) -> Vec<N>
where
    Vec<N>: FromIterator<N>,
    F: Fn(N) -> N,
{
    arr.into_iter().map(func).collect()
}

#[allow(dead_code)]
pub fn iter_reduce<N: Clone, F>(arr: Vec<N>, func: F) -> N
where
    Vec<N>: FromIterator<N>,
    F: Fn(N, N) -> N,
{
    arr.into_iter().reduce(func).unwrap()
}
