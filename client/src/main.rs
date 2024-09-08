use client;
use constrained_inputs::input;
fn main() {
    let mut client = client::MineSweeperClient::start_game("127.0.0.1:8000", (6, 6), 4)
        .expect("Unable to connect to server");

    while client.is_playing() {
        let index;
        loop {
            let res = input::<usize>();
            if res.is_ok() {
                index = res.unwrap();
                break;
            }
        }
        client.reveal_cell(index);
        client.print_board();
    }

    if client.is_won() {
        println!("Yes you won!");
    } else {
        println!("Better luck next time");
    }
}
