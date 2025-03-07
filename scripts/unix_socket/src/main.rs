use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
const SOCKET_PATH: &str = "/tmp/central.sock";
const RETRY_DELAY_MS: u64 = 500;
const MAX_RETRIES: u32 = 5;

fn parse_hex_to_u8_array(input: &str) -> Vec<u8> {
    input
        .split_whitespace() // Split the input string by spaces
        .map(|hex| u8::from_str_radix(hex, 16).expect("Invalid hex number")) // Convert each hex string to u8
        .collect() // Collect into a Vec<u8>
}
// convert the received byte array for rx packets
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().concat()
}
fn format_hex_string(input: &str) -> String {
    // Validate the input length is even
    if input.len() % 2 != 0 {
        panic!("Hex string length must be even!");
    }

    // Split the input string into 2-character chunks and join them with spaces
    input
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join(" ")
}
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
        if buffer.trim_end() == "Tx" {
            // can not send string Send raw bytes `0500`
            let input = "60 23 00 00 00 00 00 c0 02 01 06 07 03 0d 18 0f 18 05 18 11 07 f0 de bc 9a 78 56 34 12 78 56 34 12 78 56 34";
            // let input = "60 23 24 D2 5A 24 D2 5A 02 01 06 07 03 0D 18 0F 18 05 18 11 07 F0 DE BC 9A 78 56 34 12 78 56 34 12 78 56 34 12 38 7D 62";
            let response = parse_hex_to_u8_array(input);
            stream.write_all(&response)?;
            println!("Sent: {:?}",response);
        } else {
            // Default response
            // if buffer.trim_end() == "E52209A6540EE9EB0000000000C0C0AF1FA8D711FF010000280000009001FFFFFFFF1FAAF37005"{
            println!("===============Connected===============");
            let input2 = "05 00 A0 32 0D";
            let response = parse_hex_to_u8_array(input2);
            stream.write_all(&response)?;
            println!("Sent: {:?}",response);
            // }
            // // let response = format!("ACK: {}", buffer.trim_end());
            // // println!("This is the buffer trim_end: {}",buffer.trim_end());
            // else{
            // let rx_ddata = bytes_to_hex_string((buffer.trim_end()).as_bytes());
            // println!("rx_ddata: {}", rx_ddata);
            // stream.write_all(response.as_bytes())?;
            // }
    }
        stream.flush()?;
    }

    Ok(())
}

fn connect_to_server() -> io::Result<UnixStream> {
    let mut retries = 0;
    loop {
        match UnixStream::connect(SOCKET_PATH) {
            Ok(stream) => {
                println!("Connected to server at {}", SOCKET_PATH);
                return Ok(stream);
            }
            Err(e) => {
                eprintln!("Error connecting to server: {}", e);
                retries += 1;
                if retries >= MAX_RETRIES {
                    eprintln!("Max retries reached. Exiting.");
                    return Err(e);
                }
                println!("Retrying connection in {} ms...", RETRY_DELAY_MS);
                thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
            }
        }
    }
}

fn communicate_with_server(mut stream: UnixStream) -> io::Result<()> {
    // let stdin = io::stdin();
    let mut reader = BufReader::new(stream.try_clone()?);


    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer)?;

        println!("Received: {}", buffer.trim_end());

        // Respond back to the client
        if buffer.trim_end() == "Tx" {
            // can not send string Send raw bytes `0500`
            let input = "60 23 00 00 00 00 00 c0 02 01 06 07 03 0d 18 0f 18 05 18 11 07 f0 de bc 9a 78 56 34 12 78 56 34 12 78 56 34";
            // let input = "60 23 24 D2 5A 24 D2 5A 02 01 06 07 03 0D 18 0F 18 05 18 11 07 F0 DE BC 9A 78 56 34 12 78 56 34 12 78 56 34 12 38 7D 62";
            let response = parse_hex_to_u8_array(input);
            stream.write_all(&response)?;
            println!("Sent: {:?}",response);
            // break;
        } else {
            // Default response
            let response = format!("ACK: {}", buffer.trim_end());
            // println!("This is the buffer trim_end: {}",buffer.trim_end());
            let rx_ddata = bytes_to_hex_string((buffer.trim_end()).as_bytes());
            println!("rx_ddata: {}", rx_ddata);
            stream.write_all(response.as_bytes())?;
        }
        stream.flush()?;
    }

    Ok(())
}

// main for rust unix server
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
// Main for rust unix client
// fn main() {
//     match connect_to_server() {
//         Ok(stream) => {
//             if let Err(e) = communicate_with_server(stream) {
//                 eprintln!("Communication error: {}", e);
//             }
//         }
//         Err(e) => {
//             eprintln!("Failed to connect to server: {}", e);
//         }
//     }
// }