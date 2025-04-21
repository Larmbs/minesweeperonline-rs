mod board;
use board::BoardInstance;

use easy_sockets::{start_server, ServerConn};

mod message {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub enum Client {
        // size: (u16)
        // name: (error_code)
        Error(u16),
        // size: (u16)
        // name: (version)
        // If version is invalid then it throws an error.
        SetVersion(u16),
        // size: (u8, u8, u16)
        // name: (width, height, mine_count)
        // If width or height exceed 100 then throws an error.
        // If mine_count exceeds 100*100 - 1 then throws an error.
        NewGame(u8, u8, u16),
        // size: (u16)
        // name: (index)
        // If index is out of range then throws an error.
        Reveal(u16),
        // size: ()
        // name: ()
        GetTime(),
        // size: ()
        // name: ()
        CloseGame(),
    }

    #[derive(Serialize, Deserialize)]
    pub enum Server {
        // size: (u16)
        // name: (error_code)
        Error(u16),
        // size: ()
        // name: ()
        Accepted(),
        // size: ([u8; u16])
        // name: ([val; width*height])
        RevealCells(Vec<u8>),
        // size: ([u8; u16])
        // name: ([val; width*height])
        GameWin(Vec<u8>),
        // size: (Vec<u16>)
        // name: (Vec<index>)
        GameLoss(Vec<u16>),
        // size: (String)
        // name: (time)
        Time(String),
    }
}

/// Represents the games current state
#[derive(PartialEq)]
pub enum State {
    Playing,
    Idle,
    Lost,
    Won,
}

struct MineSweeperServerConn {
    pub version: u16,
    pub board: Option<BoardInstance>,
    pub state: State,
}
impl ServerConn for MineSweeperServerConn {
    type ClientMsg = message::Client;

    type ServerMsg = message::Server;

    fn handle_message(&mut self, message: Self::ClientMsg) -> Self::ServerMsg {
        match message {
            Self::ClientMsg::Error(code) => panic!("Error Code Received: {}", code),
            Self::ClientMsg::SetVersion(version) => self.set_version(version),
            Self::ClientMsg::Reveal(index) => self.reveal(index as usize),
            Self::ClientMsg::NewGame(width, height, mine_count) => {
                self.new_game(width as usize, height as usize, mine_count as usize)
            }
            Self::ClientMsg::GetTime() => Self::ServerMsg::Accepted(),
            Self::ClientMsg::CloseGame() => self.close_game(),
        }
    }

    fn new() -> Self {
        Self {
            version: 0,
            board: None,
            state: State::Idle,
        }
    }
}

impl MineSweeperServerConn {
    pub fn set_version(&mut self, version: u16) -> message::Server {
        self.version = version;
        self.close_game()
    }
    pub fn reveal(&mut self, index: usize) -> message::Server {
        if let Some(ref mut board) = self.board {
            let revealed = board.reveal_cells(index);
            if revealed.len() == 0 {
                message::Server::GameLoss(board.get_bomb_positions())
            } else if board.revealed_all() {
                message::Server::GameWin(revealed)
            } else {
                message::Server::RevealCells(revealed)
            }
        } else {
            message::Server::Error(100)
        }
    }
    pub fn new_game(&mut self, width: usize, height: usize, mine_count: usize) -> message::Server {
        self.board = Some(BoardInstance::init(&(width, height), mine_count));
        self.state = State::Playing;
        message::Server::Accepted()
    }
    pub fn close_game(&mut self) -> message::Server {
        self.state = State::Idle;
        self.board = None;
        message::Server::Accepted()
    }
}

#[tokio::main]
async fn main() {
    start_server::<MineSweeperServerConn>("127.0.0.1:8000").await.expect("Server Closed");
}
