#[allow(dead_code)]
pub fn negative(input: Vec<i32>) -> Vec<i32> {
    input.into_iter().filter(|&x| x >= 0).collect::<Vec<i32>>()
}

#[allow(dead_code)]
pub fn positive(input: Vec<i32>) -> Vec<i32> {
    input.into_iter().filter(|&x| x <= 0).collect::<Vec<i32>>()
}

#[allow(dead_code)]
pub fn odds(input: Vec<i32>) -> Vec<i32> {
    input
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<i32>>()
}

#[allow(dead_code)]
pub fn evens(input: Vec<i32>) -> Vec<i32> {
    input
        .into_iter()
        .filter(|&x| x % 2 != 0)
        .collect::<Vec<i32>>()
}
