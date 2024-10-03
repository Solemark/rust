#[allow(dead_code)]
pub fn negative(input: Vec<i32>) -> Vec<i32> {
    input.into_iter().filter(|&x| x.is_positive()).collect()
}

#[allow(dead_code)]
pub fn positive(input: Vec<i32>) -> Vec<i32> {
    input.into_iter().filter(|&x| x.is_negative()).collect()
}

#[allow(dead_code)]
pub fn odds(input: Vec<i32>) -> Vec<i32> {
    input.into_iter().filter(|&x| x % 2 == 0).collect()
}

#[allow(dead_code)]
pub fn evens(input: Vec<i32>) -> Vec<i32> {
    input.into_iter().filter(|&x| x % 2 != 0).collect()
}
