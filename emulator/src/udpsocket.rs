use std::net::UdpSocket;

pub fn udp_socket(message: &str) -> std::io::Result<()> {
    // Bind the socket to a local address
    let socket = UdpSocket::bind("127.0.0.1:8888")?;

    // Set the destination address (you can change the port)
    let target_addr = "127.0.0.1:6666";

    // Send the message
    socket.send_to(message.as_bytes(), target_addr)?;

    println!("Sent message: {}", message);
    Ok(())
}