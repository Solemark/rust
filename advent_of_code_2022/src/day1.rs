#[allow(dead_code)]
pub fn day1(data: Vec<String>) -> String {
    let inp: Vec<i32> = data.into_iter().map(|s| s.parse().unwrap_or(0)).collect();
    let last: i32 = inp.len().try_into().unwrap();
    let mut total: i32 = 0;
    let mut highest: (usize, i32) = (1, 0);
    for i in inp {
        if i == last - 1 && total > highest.1 {
            highest.0 += 1;
            highest.1 = total;
        }
        if i == 0 {
            if total > highest.1 {
                highest.0 += 1;
                highest.1 = total;
            }
            total = 0;
        }
        total += i;
    }
    format!("Elf {} has the most calories", highest.0)
}
