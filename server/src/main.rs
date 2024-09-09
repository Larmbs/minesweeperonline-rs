use tokio::io::{split, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
mod board;
mod protocol_v2;
use protocol_v2::{ClientMsg, ServerMsg};
mod zip;
use board::BoardInstance;

/// Represents the games current state
#[derive(PartialEq)]
pub enum State {
    Playing,
    Idle,
    Lost,
    Won,
}
struct ClientHandler {
    pub version: u16,
    pub board: Option<BoardInstance>,
    pub state: State,
}
impl ClientHandler {
    pub fn new() -> Self {
        ClientHandler {
            version: 0,
            board: None,
            state: State::Idle,
        }
    }
    pub fn set_version(&mut self, version: u16) -> ServerMsg {
        self.version = version;
        self.board = None;
        self.state = State::Idle;
        ServerMsg::Accepted()
    }
    pub fn reveal(&mut self, index: usize) -> ServerMsg {
        if let Some(ref mut board) = self.board {
            let revealed = board.reveal_cells(index);
            if revealed.len() == 0 {
                ServerMsg::GameLoss(board.get_bomb_positions())
            } else if board.revealed_all() {
                ServerMsg::GameWin(revealed)
            } else {
                ServerMsg::RevealCells(revealed)
            }
        } else {
            ServerMsg::Error(100)
        }
    }
    pub fn new_game(&mut self, width: usize, height: usize, mine_count: usize) -> ServerMsg {
        self.board = Some(BoardInstance::init(&(width, height), mine_count));
        self.state = State::Playing;
        ServerMsg::Accepted()
    }
    pub fn close_game(&mut self) -> ServerMsg {
        self.state = State::Idle;
        self.board = None;
        ServerMsg::Accepted()
    }
}

pub async fn handle(mut socket: TcpStream) {
    let (reader, mut writer) = split(&mut socket);
    let mut reader = BufReader::new(reader);
    let mut buffer = vec![0; 2048];

    let mut client_handler = ClientHandler::new();
    loop {
        match reader.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                let msg = ClientMsg::from_bytes(&buffer).expect("");
                let response: ServerMsg = match msg {
                    ClientMsg::Error(code) => panic!("Error Code Received: {}", code),
                    ClientMsg::SetVersion(version) => {
                        client_handler.set_version(version)
                    },
                    ClientMsg::Reveal(index) => {
                        client_handler.reveal(index as usize)
                    }
                    ClientMsg::NewGame(width, height, mine_count) => {
                        client_handler.new_game(width as usize, height as usize, mine_count as usize)
                    },
                    ClientMsg::GetTime() => ServerMsg::Accepted(),
                    ClientMsg::CloseGame() => {
                        client_handler.close_game()
                    },
                };

                let bytes = response.to_bytes().unwrap();
                writer
                    .write_all(&bytes)
                    .await
                    .expect("Failed to write to socket");
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound
                | std::io::ErrorKind::PermissionDenied
                | std::io::ErrorKind::ConnectionRefused
                | std::io::ErrorKind::ConnectionReset
                | std::io::ErrorKind::ConnectionAborted
                | std::io::ErrorKind::NotConnected
                | std::io::ErrorKind::AddrNotAvailable
                | std::io::ErrorKind::BrokenPipe
                | std::io::ErrorKind::AlreadyExists
                | std::io::ErrorKind::TimedOut => panic!("{:?}", err.kind()),
                _ => continue,
            },
        }
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
