use common::{self, handle_response, send_message, write_to_file, COORD_ROUTE, MOVIE_ROUTE};
use std::net::TcpListener;

fn main() {
    for stream in TcpListener::bind(MOVIE_ROUTE).unwrap().incoming() {
        let msg = handle_response(stream.unwrap());
        println!("recieved message: {}", msg);

        write_to_file(&msg, "movie");
        send_message("cmovie successfully written", COORD_ROUTE);
    }
}
