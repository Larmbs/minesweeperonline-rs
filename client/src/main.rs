use client;

fn main() {
    let mut client = client::MineSweeperClient::start_game("127.0.0.1:8000", (8, 8), 20)
        .expect("Unable to connect to server");
    client.reveal_cell(32);
    println!("{:?}", client);
}
