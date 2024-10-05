use common::{
    handle_response, send_message, BIND_ERR, BOOK_ROUTE, CLIENT_ROUTE, COORD_ROUTE, MOVIE_ROUTE,
};
use std::net::TcpListener;

fn main() {
    for stream in TcpListener::bind(COORD_ROUTE).expect(BIND_ERR).incoming() {
        let msg = handle_response(stream.unwrap());
        println!("recieved message: {}", msg);

        handle_message(&msg);
    }
}

fn handle_message(msg: &str) {
    let mut chars = msg.chars();
    match chars.next().unwrap() {
        'b' => send_message(chars.as_str(), BOOK_ROUTE),
        'm' => send_message(chars.as_str(), MOVIE_ROUTE),
        'c' => send_message(chars.as_str(), CLIENT_ROUTE),
        _ => send_message("unknown type!", CLIENT_ROUTE),
    }
}
