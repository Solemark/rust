use common::{self, handle_response, send_message, write_to_file, BOOK_ROUTE, COORD_ROUTE};
use std::net::TcpListener;

fn main() {
    for stream in TcpListener::bind(BOOK_ROUTE).unwrap().incoming() {
        let msg = handle_response(stream.unwrap());
        println!("recieved message: {}", msg);

        write_to_file(&msg, "book");
        send_message("cbook successfully written", COORD_ROUTE);
    }
}
