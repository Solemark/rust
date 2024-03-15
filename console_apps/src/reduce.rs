#[allow(dead_code)]
pub fn red_add<N: std::ops::Add<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a + b).unwrap()
}

#[allow(dead_code)]
pub fn red_sub<N: std::ops::Sub<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a - b).unwrap()
}

#[allow(dead_code)]
pub fn red_mul<N: std::ops::Mul<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a * b).unwrap()
}

#[allow(dead_code)]
pub fn red_div<N: std::ops::Div<Output = N>>(input: Vec<N>) -> N {
    input.into_iter().reduce(|a, b| a / b).unwrap()
}
