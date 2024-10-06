use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub const CLIENT_ROUTE: &str = "127.0.0.1:5000";
pub const COORD_ROUTE: &str = "127.0.0.1:5001";
pub const BOOK_ROUTE: &str = "127.0.0.1:5003";
pub const MOVIE_ROUTE: &str = "127.0.0.1:5002";

pub const BIND_ERR: &str = "TcpListener::bind failed";
pub const CONN_ERR: &str = "TcpStream::connect failed";
pub const WRTE_ERR: &str = "TcpStream::write failed";
pub const READ_ERR: &str = "TcpStream::read failed";

pub const FLSH_ERR: &str = "Stdout::flush failed";
pub const RDLN_ERR: &str = "Stdin::read_line failed";

pub fn send_and_recv(msg: &str, send_addr: &str, resp_addr: &str) -> String {
    let mut res = String::new();
    send_message(msg, send_addr);

    for stream in TcpListener::bind(resp_addr).expect(BIND_ERR).incoming() {
        res = handle_response(stream.unwrap());
        break;
    }
    res
}

pub fn send_message(msg: &str, addr: &str) {
    TcpStream::connect(addr)
        .expect(CONN_ERR)
        .write(msg.as_bytes())
        .expect(WRTE_ERR);
}

pub fn handle_response(mut stream: TcpStream) -> String {
    let mut msg = [0; 1024];
    stream.read(&mut msg).expect(READ_ERR);
    String::from_utf8_lossy(&msg).to_string()
}

pub fn write_to_file(msg: &str, name: &str) {
    let mut output = msg.chars();
    output.next();
    println!("{} writing {}", name, output.as_str());
}
