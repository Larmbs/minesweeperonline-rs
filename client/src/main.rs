use client;

fn main() {
    let mut client = client::MineSweeperClient::start_game("127.0.0.1:8000", (6, 6), 4)
        .expect("Unable to connect to server");
    client.reveal_cell(1);
    client.reveal_cell(16);
}
