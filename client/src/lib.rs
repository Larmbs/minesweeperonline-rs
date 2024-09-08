mod protocol;

use anyhow::Result;
use protocol::{try_send, MsgReceive, MsgSend};
use std::net::{TcpStream, ToSocketAddrs};

/// Represents an individual MineSweeper cell's state
#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Revealed(u8),
    Hidden(bool),
}

/// A MineSweeper client to interact with server online
#[derive(Debug)]
pub struct MineSweeperClient {
    socket: TcpStream,
    dim: (usize, usize),
    cells: Vec<Cell>,
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
            })
        } else {
            todo!()
        }
    }

    /// Reveals a cell
    pub fn reveal_cell(&mut self, index: usize) {
        assert!(index < self.cells.len(), "Index provided is out of range");
        if self.cells[index] == Cell::Hidden(false) {
            let reply = try_send(&mut self.socket, MsgSend::Reveal(index)).unwrap();
            match reply {
                MsgReceive::Error(msg) => panic!("{}", msg),
                MsgReceive::ConnectionAccepted => panic!("Should never happen"),
                MsgReceive::RevealCells(cells) => {
                    for (index, value) in cells {
                        assert!(index < self.cells.len(), "Server supplied fatal indices");
                        self.cells[index] = Cell::Revealed(value);
                    }
                }
                MsgReceive::GameEnd(_, _, _) => todo!(),
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
}
