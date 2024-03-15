fn double(num: i8) -> i8 {
    num * 2
}

fn main() {
    let input: Vec<i8> = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!(
        "Mapped: {:?}",
        input.clone().into_iter().map(double).collect::<Vec<i8>>()
    );
    println!(
        "Filtered: {:?}",
        input
            .clone()
            .into_iter()
            .filter(|&x| x > 0)
            .collect::<Vec<i8>>()
    );
    println!(
        "Reduced: {}",
        input.clone().into_iter().reduce(|a, b| a + b).unwrap()
    );
}
