fn main() {
    println!("{}", parse_sql("select 'Hi Mom!';".to_lowercase()));
    println!("{}", parse_sql("select * from clients;".to_lowercase()));
}

fn parse_sql(sql: String) -> String {
    if !check_ends(&sql) {
        return String::from("Malformed query");
    }
    if only_select(&sql) {
        let res = &sql[7..];
        return res.replace("'", "").replace(";", "");
    }
    let (s, f) = split_query(&sql);
    let select = clean_select(s);
    let from = clean_from(f);
    format!("{}\n{}", select, from)
}

fn check_ends(sql: &String) -> bool {
    let start = &sql[..6];
    if !vec!["select", "update"].contains(&start) {
        return false;
    }

    let end = &sql.chars().last().unwrap();
    if *end != ';' {
        return false;
    }
    return true;
}

fn only_select(sql: &String) -> bool {
    if sql.contains("select") {
        if !sql.contains("from") {
            return true;
        }
    }
    false
}

fn split_query(sql: &String) -> (String, String) {
    let split = sql.split(" from").collect::<Vec<&str>>();
    (String::from(split[0]), String::from(split[1]))
}

fn clean_select(select: String) -> String {
    String::from(&select[7..])
}

fn clean_from(from: String) -> String {
    from.replace(" ", "").replace(";", "")
}
