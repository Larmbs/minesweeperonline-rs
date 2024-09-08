use std::io;
use std::net;
mod protocol;
use protocol::{send, MsgReceive, MsgSend};

#[derive(Clone, PartialEq)]
enum Cell {
    REVEALED(u8),
    HIDDEN(bool),
}
struct MineSweeperClient {
    socket: net::TcpStream,
    dim: (usize, usize),
    cells: Vec<Cell>,
}
impl MineSweeperClient {
    pub fn start_game<A: net::ToSocketAddrs>(
        server_addr: A,
        dim: (usize, usize),
        mine_count: usize,
    ) -> io::Result<Self> {
        let mut socket = net::TcpStream::connect(server_addr)?;

        let reply = send(&mut socket, MsgSend::Connect(dim, mine_count));

        if reply == MsgReceive::ConnectionAccepted {
            Ok(Self {
                socket,
                dim,
                cells: vec![Cell::HIDDEN(false); dim.0 * dim.1],
            })
        } else {
            todo!()
        }
    }
    pub fn reveal_cell(&mut self, index: usize) {
        assert!(index < self.cells.len(), "Index provided is out of range");
        if self.cells[index] == Cell::HIDDEN(false) {
            let reply = send(&mut self.socket, MsgSend::Reveal(index));
            match reply {
                MsgReceive::Error(msg) => panic!("{}", msg),
                MsgReceive::ConnectionAccepted => panic!("Should never happen"),
                MsgReceive::RevealCells(cells) => {
                    for (index, value) in cells {
                        assert!(index < self.cells.len(), "Server supplied fatal indices");
                        self.cells[index] = Cell::REVEALED(value);
                    }
                }
                MsgReceive::GameEnd(_, _, _) => todo!(),
            }
        }
    }
    pub fn flag_cell(&mut self, index: usize) {
        assert!(index < self.cells.len(), "Index provided is out of range");
        if let Cell::HIDDEN(ref mut val) = self.cells[index] {
            *val = !*val;
        }
    }
}

fn main() {
    let mut game = MineSweeperClient::start_game("127.0.0.1:8000", (8, 8), 20)
        .expect("Unable to connect to server");
    game.reveal_cell(4);
    println!("Hello, world!");
}
