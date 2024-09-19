use std::net::{UdpSocket, SocketAddr};
use std::error::Error;
use std::str;

fn main() -> Result<(),Box<dyn Error>> {
    // Bind the socket to a local address and port
    let socket = UdpSocket::bind("127.0.0.1:9999")?; // 0.0.0.0:0 means any available port

    // Define the remote address and port to which you want to send the message
    let remote_addr: SocketAddr = "127.0.0.1:7777".parse()?;

    // The message to send
    let msg = b"Hello, world!";

    // Send the message to the remote address
    socket.send_to(msg, remote_addr)?;

    // Vector to hold all received bytes

    loop{
        let mut all_bytes: Vec<u8> = Vec::new();

        let mut buf = [0; 1024]; // Buffer for receiving data
        // Receive a message from any source
        let (amt, src) = socket.recv_from(&mut buf)?;

        all_bytes.extend_from_slice(&buf[..amt]);
        // Convert the received byte array to a hex string
        let hex_string: String = all_bytes
            .iter()
            .map(|byte| format!("0x{:02X}", byte)) // Convert each byte to hex
            .collect::<Vec<String>>()
            .join(" "); // Join with spaces for readability

        // Print the result
        println!("Received {} bytes from {}: {}", amt, src, hex_string);
        // You can also send a reply to the source if needed
        let reply = b"Message received!";
        socket.send_to(reply, src)?;
    }
    // loop{
    //     let mut buf = [0; 1024]; // Buffer for receiving data
    //     // Receive a message from any source
    //     let (amt, src) = socket.recv_from(&mut buf)?;

    //     // Print the received message and the source
    //     let received_msg = str::from_utf8(&buf[..amt]).unwrap_or("Invalid UTF-8");
    //     println!("Received {} bytes from {}: {}", amt, src, received_msg);

    //     // You can also send a reply to the source if needed
    //     let reply = b"Message received!";
    //     socket.send_to(reply, src)?;
    // }
}