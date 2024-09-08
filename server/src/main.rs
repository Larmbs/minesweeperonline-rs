use tokio::io::{split, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
mod protocol;
use anyhow::Result;
use protocol::{MsgReceive, MsgSend};

enum Cell {
    // Represents a cell opened by the client
    REVEALED(u8),
    // Represents a hidden cell with a flag representing if its occupied by a bomb
    HIDDEN(bool),
}

struct MineSweeperServerConnection {
    socket: TcpStream,
    dim: (usize, usize),
    cells: Vec<Cell>,
}

impl MineSweeperServerConnection {
    pub async fn accept(mut socket: TcpStream) -> Result<Self> {
        let mut buffer = vec![0u8; 1024];

        // Use async read for non-blocking
        socket.read(&mut buffer).await?;

        // Deserialize the message safely with error handling
        let msg: MsgReceive = match buffer.try_into() {
            Ok(m) => m,
            Err(_) => return Err(anyhow::anyhow!("Failed to parse message")),
        };

        match msg {
            MsgReceive::Error(msg) => panic!("{}", msg),
            MsgReceive::Reveal(_) => panic!("Should not be receiving this"),
            MsgReceive::Connect((width, height), _mine_count) => {
                // Send a connection accepted message
                let msg: Vec<u8> = MsgSend::ConnectionAccepted.try_into().unwrap();
                socket.write_all(&msg).await?;

                // Initialize the connection
                Ok(Self {
                    socket,
                    dim: (width, height),
                    cells: vec![],
                })
            }
        }
    }

    pub async fn handle(&mut self) {
        let (reader, mut writer) = split(&mut self.socket);
        let mut reader = BufReader::new(reader);
        let mut buffer = vec![0; 1024];

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .await
                .expect("Failed to read from socket");
            if bytes_read == 0 {
                // Connection closed
                break;
            }
            
            // Process the incoming data (placeholder for actual game logic)
            writer
                .write_all(b"Message received")
                .await
                .expect("Failed to write to socket");
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Error starting the server");

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        tokio::spawn(async move {
            // Handle the connection asynchronously, using `.await`
            match MineSweeperServerConnection::accept(socket).await {
                Ok(mut conn) => {
                    conn.handle().await;
                }
                Err(e) => eprintln!("Connection error: {:?}", e),
            }
        });
    }
}
