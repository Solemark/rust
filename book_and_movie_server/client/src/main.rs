use common::{send_and_recv, CLIENT_ROUTE, COORD_ROUTE, FLSH_ERR, RDLN_ERR};
use std::io::{stdin, stdout, Write};

fn main() {
    let msg = get_message();
    println!("message: {}", msg);

    let res = send_and_recv(&msg, COORD_ROUTE, CLIENT_ROUTE);
    println!("{}", res)
}

fn get_message() -> String {
    format!(
        "{},{},{},{}",
        ask("Enter item type: "),
        ask("Enter item name: "),
        ask("Enter item quantity: "),
        ask("Enter item price: ")
    )
}

fn ask(msg: &str) -> String {
    let mut output = String::new();
    println!("{}", msg);

    stdout().flush().expect(FLSH_ERR);
    stdin().read_line(&mut output).expect(RDLN_ERR);

    output.trim().to_string()
}
