#[allow(dead_code)]
pub fn add<N: std::ops::Add<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a + b).unwrap()
}

#[allow(dead_code)]
pub fn sub<N: std::ops::Sub<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a - b).unwrap()
}

#[allow(dead_code)]
pub fn mul<N: std::ops::Mul<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a * b).unwrap()
}

#[allow(dead_code)]
pub fn div<N: std::ops::Div<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a / b).unwrap()
}

#[allow(dead_code)]
pub fn other<N>(input: Vec<N>, func: fn(N, N) -> N) -> N {
    input.into_iter().reduce(func).unwrap()
}
