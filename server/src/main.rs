use tokio::io::{split, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
mod protocol;
use protocol::{MsgReceive, MsgSend};
mod board;
use board::BoardInstance;
use chrono;

pub async fn handle(mut socket: TcpStream) {
    let (reader, mut writer) = split(&mut socket);
    let mut reader = BufReader::new(reader);
    let mut buffer = vec![0; 1024];

    let start_time = chrono::Utc::now();
    let mut board_instance: Option<BoardInstance> = None;
    let mut running = true;
    while running {
        if reader
            .read(&mut buffer)
            .await
            .expect("Failed to read from socket")
            == 0
        {
            break;
        };

        let msg = MsgReceive::try_from(&buffer).unwrap();
        let response = match msg {
            MsgReceive::Error(msg) => panic!("{}", msg),
            MsgReceive::Connect(dim, mine_count) => {
                if let Some(ref _board) = board_instance {
                    MsgSend::Error("Client already created a board".to_string())
                } else {
                    board_instance = Some(BoardInstance::init(&dim, mine_count));

                    MsgSend::ConnectionAccepted
                }
            }
            MsgReceive::Reveal(index) => {
                if let Some(ref mut board) = board_instance {
                    if index < board.cells.len() {
                        let revealed_cells = board.reveal(index);
                        if revealed_cells.len() == 0 {
                            running = false;
                            let delta_time = chrono::Utc::now() - start_time;
                            MsgSend::GameLoss(delta_time.to_string(), board.get_bomb_positions())
                        } else {
                            if board.revealed_all() {
                                running = false;
                                let delta_time = chrono::Utc::now() - start_time;
                                MsgSend::GameWin(delta_time.to_string(), revealed_cells)
                            } else {
                                MsgSend::RevealCells(revealed_cells)
                            }
                        }
                    } else {
                        MsgSend::Error("Client provided index out of bounds".to_string())
                    }
                } else {
                    MsgSend::Error("Client did not initially connect to server".to_string())
                }
            }
        };

        let bytes: Vec<u8> = response.try_into().unwrap();
        writer
            .write_all(&bytes)
            .await
            .expect("Failed to write to socket");
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Server");
    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Error starting the server");

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        tokio::spawn(async move {
            println!("Received Connection");
            handle(socket).await;
            println!("Connection complete")
        });
    }
}
