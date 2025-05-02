use std::net::UdpSocket;

pub fn udp_socket(message: &str, port: u16) -> std::io::Result<()> {
    // Bind the socket to a local address
    let socket = UdpSocket::bind("127.0.0.1:8888")?;

    // Set the destination address (you can change the port)
    let target_addr = format!("127.0.0.1:{}",port);

    // Send the message
    socket.send_to(message.as_bytes(), &target_addr)?;

    println!("Sent message: {}", message);
    Ok(())
}
