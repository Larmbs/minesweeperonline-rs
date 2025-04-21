//! Defines a client which can interact with MineSweeper server
mod protocol;
mod protocol_v2;
mod zip;

use anyhow::Result;
use protocol_v2::{Bytes, ClientMsg, ServerMsg, MAX_BYTES};
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};
use easy_sockets::prelude::*;

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


/// Represents an individual MineSweeper cell's state
#[derive(Clone, PartialEq)]
pub enum Cell {
    Revealed(u8),
    Hidden(bool),
    Mine,
    MineExploded,
}

/// Represents the games current state
#[derive(PartialEq, Debug)]
pub enum State {
    Connecting,
    Playing,
    Idle,
    Lost,
    Won,
}
impl State {
    pub fn should_display(&self) -> bool {
        match self {
            State::Playing | State::Lost | State::Won => {
                true
            },
            _ => false,
        }
    }
}
#[derive(Clone)]
pub struct Board {
    pub dim: (usize, usize),
    pub cells: Vec<Cell>,
}
impl Board {
    pub fn new(dim: (usize, usize)) -> Board {
        Board {
            dim,
            cells: vec![Cell::Hidden(false); dim.0 * dim.1],
        }
    }
    pub fn reveal_cells(&mut self, cells: &Vec<u8>) {
        assert!(
            cells.len() == self.cells.len(),
            "Too many cells were provided"
        );
        for (i, v) in cells.iter().enumerate() {
            match v {
                0..=8 => self.cells[i] = Cell::Revealed(*v),
                _ => (),
            }
        }
    }

    /// Flags a cell for convenience
    pub fn flag_cell(&mut self, index: usize) {
        assert!(index < self.cells.len(), "Index provided is out of range");
        if let Cell::Hidden(ref mut val) = self.cells[index] {
            *val = !*val;
        }
    }

    /// Reveals all hidden cells as mines
    pub fn reveal_all_as_mines(&mut self) {
        for i in 0..self.cells.len() {
            if let Cell::Hidden(_) = self.cells[i] {
                self.cells[i] = Cell::Mine;
            }
        }
    }

    /// Show Mines
    pub fn show_mines(&mut self, mines: &Vec<u16>) {
        for i in mines {
            self.cells[*i as usize] = Cell::MineExploded;
        }
    }
}

/// A MineSweeper client to interact with server online
pub struct MineSweeperClient {
    socket: TcpStream,
    error_code: u16,
    pub state: State,
    pub board: Option<Board>,
}
impl tcp::ClientTCP for MineSweeperClient {
    type ClientMsg = message::Client;

    type ServerMsg = message::Server;

    fn handle_response(&mut self, response: Self::ServerMsg) {
        match response {
            message::Server::Error(_) => todo!(),
            message::Server::Accepted() => todo!(),
            message::Server::RevealCells(vec) => todo!(),
            message::Server::GameWin(vec) => todo!(),
            message::Server::GameLoss(vec) => todo!(),
            message::Server::Time(_) => todo!(),
        }
    }

    fn update(&mut self) -> Option<()> {
        if self.state == State::Connecting {
            self.send_message(message::Client::SetVersion(2)).expect("Error connecting to server")
        }

        Some(())
    }
}
impl MineSweeperClient {
    /// Starts a game by connecting to server
    pub fn connect<A: ToSocketAddrs>(server_addr: A) -> Result<Self> {
        let mut socket = TcpStream::connect(server_addr)?;

        let reply = Self::send_message(&mut socket, ClientMsg::SetVersion(2))
            .expect("Failed opening message");

        if reply == ServerMsg::Accepted() {
            Ok(Self {
                socket,
                error_code: 200,
                state: State::Idle,
                board: None,
            })
        } else {
            panic!("Should never happen")
        }
    }
    pub fn new_game(&mut self, dim: (usize, usize), mine_count: usize) {
        let reply = Self::send_message(
            &mut self.socket,
            ClientMsg::NewGame(dim.0 as u8, dim.1 as u8, mine_count as u16),
        )
        .expect("Failed opening message");

        if reply == ServerMsg::Accepted() {
            self.board = Some(Board::new(dim));
            self.state = State::Playing;
        }
    }

    pub fn close_game(&mut self) {
        let reply = Self::send_message(&mut self.socket, ClientMsg::CloseGame()).unwrap();

        if reply == ServerMsg::Accepted() {
            self.board = None;
            self.state = State::Idle;
        }
    }

    fn send_message(socket: &mut TcpStream, message: ClientMsg) -> Result<ServerMsg> {
        let bytes = message.to_bytes()?;
        socket.write_all(&bytes)?;

        let mut buffer: Bytes = vec![0; MAX_BYTES];
        socket.read(&mut buffer)?;
        let reply = ServerMsg::from_bytes(&buffer)?;
        Ok(reply)
    }

    /// Reveals a cell
    pub fn reveal_cell(&mut self, index: usize) {
        if let Some(ref mut board) = self.board {
            if self.state == State::Playing {
                let reply = Self::send_message(&mut self.socket, ClientMsg::Reveal(index as u16))
                    .expect("Failed to send message");

                match reply {
                    ServerMsg::Error(code) => {
                        self.error_code = code;
                    },
                    ServerMsg::RevealCells(cells) => {
                        board.reveal_cells(&cells);
                    },
                    ServerMsg::GameWin(cells) => {
                        board.reveal_cells(&cells);
                        board.reveal_all_as_mines();
                        self.state = State::Won;
                    },
                    ServerMsg::GameLoss(mines) => {
                        board.show_mines(&mines);
                        self.state = State::Lost;
                    },
                    _ => panic!("Invalid response received"),
                }
            }
        }
    }

    /// Flags a cell for convenience
    pub fn flag_cell(&mut self, index: usize) {
        if self.state == State::Playing {
            if let Some(ref mut board) = self.board {
                board.flag_cell(index);
            }
        }
    }
}
