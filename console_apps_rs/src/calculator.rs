#[allow(dead_code)]
pub fn add<N: std::ops::Add<Output = N>>(x: N, y: N) -> N {
    x + y
}

#[allow(dead_code)]
pub fn sub<N: std::ops::Sub<Output = N>>(x: N, y: N) -> N {
    x - y
}

#[allow(dead_code)]
pub fn mul<N: std::ops::Mul<Output = N>>(x: N, y: N) -> N {
    x * y
}

#[allow(dead_code)]
pub fn div<N: std::ops::Div<Output = N>>(x: N, y: N) -> N {
    x / y
}
