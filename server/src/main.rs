use std::net;
use std::io::{Write, Read};
mod protocol;
enum Cell {
    REVEALED(u8),
    HIDDEN(bool),
}
struct Game {
    socket: net::TcpStream,
    dim: (usize, usize),
    cells: Vec<Cell>,
    mines: Vec<bool>,
}

fn main() {
    let socket_listener = net::TcpListener::bind("127.0.0.1:8000").expect("Error starting the server");

    for stream in socket_listener.incoming() {
    }
}   
