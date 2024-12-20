use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, BufRead, BufReader, Write};
use std::thread;

const SOCKET_PATH: &str = "/tmp/central.sock";

fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    println!("Client connected.");

    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer)?;
        
        if bytes_read == 0 {
            // Client closed the connection
            println!("Client disconnected.");
            break;
        }

        println!("Received: {}", buffer.trim_end());

        // Respond back to the client
        let response = if buffer == "Tx\n" {
            "Tx packet".to_string()
        } else {
            format!("ACK: {}", buffer)
        };
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Ensure the socket file does not exist before starting the server
    std::fs::remove_file(SOCKET_PATH).ok();

    // Create a Unix domain socket listener
    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("Unix socket server is listening on {}", SOCKET_PATH);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a thread to handle each client connection
                thread::spawn(|| {
                    if let Err(err) = handle_client(stream) {
                        eprintln!("Error handling client: {}", err);
                    }
                });
            }
            Err(err) => {
                eprintln!("Failed to accept connection: {}", err);
            }
        }
    }

    Ok(())
}