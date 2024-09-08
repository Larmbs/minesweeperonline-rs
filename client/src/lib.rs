mod protocol;

use anyhow::Result;
use protocol::{try_send, MsgReceive, MsgSend};
use std::fmt::Debug;
use std::net::{TcpStream, ToSocketAddrs};

/// Represents an individual MineSweeper cell's state
#[derive(Clone, PartialEq)]
enum Cell {
    Revealed(u8),
    Hidden(bool),
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Revealed(val) =>
                    if *val == 0 {
                        " ".to_string()
                    } else {
                        format!("{}", val)
                    },
                Cell::Hidden(flagged) =>
                    if *flagged {
                        "f".to_string()
                    } else {
                        "+".to_string()
                    },
            }
        )
    }
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
        if self.cells[index] == Cell::Hidden(false) {
            let reply = try_send(&mut self.socket, MsgSend::Reveal(index)).unwrap();
            match reply {
                MsgReceive::Error(msg) => panic!("{}", msg),
                MsgReceive::ConnectionAccepted => panic!("Should never happen"),
                MsgReceive::RevealCells(cells) => {
                    self.reveal_cells(cells);
                }
                MsgReceive::GameWin(time, cells) => {
                    self.reveal_cells(cells);
                    println!("Time:{}", time);
                }
                MsgReceive::GameLoss(time, mines) => println!("Time:{} Mines:{:?}", time, mines),
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
