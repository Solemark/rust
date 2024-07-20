use std::collections::HashMap;

#[allow(dead_code)]
pub fn day3(data: Vec<String>) -> i32 {
    data.into_iter()
        .map(split_string)
        .reduce(|x, y| x + y)
        .unwrap_or_default()
}

fn split_string(s: String) -> i32 {
    let mut output = 0;
    let mut chars = vec![];
    let ls = split_str(0, ((s.len() / 2) - 1) as i32, &s);
    let rs = split_str((s.len() / 2) as i32, (s.len() - 1) as i32, &s);
    for c in ls.chars() {
        output += calc(c, &rs, &mut chars);
    }
    output
}

fn split_str(mut i: i32, end: i32, str: &String) -> String {
    let mut o: String = String::new();
    for c in str.chars() {
        if i > end {
            break;
        }
        o.push(c);
        i += 1;
    }
    o
}

fn calc(c: char, rs: &String, chars: &mut Vec<char>) -> i32 {
    if rs.contains(c) && !chars.contains(&c) {
        chars.push(c);
        return get_points(c);
    }
    0
}

fn get_points(c: char) -> i32 {
    let alphabet = get_alphabet();
    if c.is_uppercase() {
        return alphabet[&c.to_lowercase().last().expect("invalid character")] + 26;
    } else {
        return alphabet[&c];
    }
}

fn get_alphabet() -> HashMap<char, i32> {
    HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
    ])
}
