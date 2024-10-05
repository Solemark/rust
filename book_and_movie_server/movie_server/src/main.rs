use common::{self, handle_response, send_message, COORD_ROUTE, MOVIE_ROUTE};
use std::net::TcpListener;

fn main() {
    for stream in TcpListener::bind(MOVIE_ROUTE).unwrap().incoming() {
        let msg = handle_response(stream.unwrap());
        println!("recieved message: {}", msg);

        handle_message(&msg);
        send_message("cmovie successfully written", COORD_ROUTE);
    }
}

fn handle_message(msg: &String) {
    println!("Writing {}", msg);
}
