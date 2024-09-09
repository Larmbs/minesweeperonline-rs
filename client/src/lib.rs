//! Defines a client which can interact with MineSweeper server
mod protocol;
mod zip;

use anyhow::Result;
use protocol::{try_send, MsgReceive, MsgSend};
use std::net::{TcpStream, ToSocketAddrs};

/// Represents an individual MineSweeper cell's state
#[derive(Clone, PartialEq)]
pub enum Cell {
    Revealed(u8),
    Hidden(bool),
    Mine,
}

/// Represents the games current state
#[derive(PartialEq)]
pub enum State {
    Playing,
    Lost(String),
    Won(String),
}

/// A MineSweeper client to interact with server online
pub struct MineSweeperClient {
    socket: TcpStream,
    dim: (usize, usize),
    cells: Vec<Cell>,
    pub state: State,
}
impl MineSweeperClient {
    /// Starts a game by connecting to server
    pub fn start_game<A: ToSocketAddrs>(
        server_addr: A,
        dim: (usize, usize),
        mine_count: usize,
    ) -> Result<Self> {
        let mut socket = TcpStream::connect(server_addr)?;

        let reply = try_send(&mut socket, MsgSend::Connect(dim, mine_count)).unwrap();

        if reply == MsgReceive::ConnectionAccepted {
            Ok(Self {
                socket,
                dim,
                cells: vec![Cell::Hidden(false); dim.0 * dim.1],
                state: State::Playing,
            })
        } else {
            panic!("Should never happen")
        }
    }
    fn reveal_cells(&mut self, cells: Vec<(usize, u8)>) {
        for (index, value) in cells {
            if index >= self.cells.len() {
                let _ = try_send(
                    &mut self.socket,
                    MsgSend::Error("Server supplied indices out of bounds".to_string()),
                );
            };
            self.cells[index] = Cell::Revealed(value);
        }
    }

    /// Reveals a cell
    pub fn reveal_cell(&mut self, index: usize) {
        assert!(index < self.cells.len(), "Index provided is out of range");
        if self.state == State::Playing {
            if self.cells[index] == Cell::Hidden(false) {
                let reply = match try_send(&mut self.socket, MsgSend::Reveal(index)) {
                    Ok(reply) => reply,
                    Err(err) => {println!("{}", err); return ();},
                };
                match reply {
                    MsgReceive::Error(msg) => println!("{}", msg),
                    MsgReceive::ConnectionAccepted => panic!("Should never happen"),
                    MsgReceive::RevealCells(cells) => {
                        self.reveal_cells(cells);
                    }
                    MsgReceive::GameWin(time, cells) => {
                        self.reveal_cells(cells);
                        self.state = State::Won(format!("Time:{}", time));
                        for i in 0..self.cells.len() {
                            if let Cell::Hidden(_) = self.cells[i] {
                                self.cells[i] = Cell::Mine;
                            }
                        }
                    }
                    MsgReceive::GameLoss(time, mines) => {
                        self.state = State::Lost(format!("Time:{}", time));
                        for mine in mines {
                            self.cells[mine] = Cell::Mine;
                        }
                    }
                }
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

    /// Returns the boards cell dimensions
    pub fn get_dim(&self) -> &(usize, usize) {
        &self.dim
    }

    /// Given a coord it returns the corresponding cell index
    pub fn ix(&self, i: usize, j: usize) -> usize {
        assert!(i < self.dim.0 && j < self.dim.1, "Index out of bounds");
        i + j * self.dim.0
    }

    pub fn get_cell(&self, index: usize) -> &Cell {
        &self.cells[index]
    }
}
