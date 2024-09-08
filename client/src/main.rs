use client;

fn main() {
    let mut game = client::MineSweeperClient::start_game("127.0.0.1:8000", (8, 8), 20)
        .expect("Unable to connect to server");
    game.reveal_cell(4);
    println!("Hello, world!");
}
