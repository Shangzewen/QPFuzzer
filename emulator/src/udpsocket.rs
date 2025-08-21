use std::net::UdpSocket;

// use common::hashbrown::HashSet;
use std::collections::HashSet;


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

pub fn calcu_branch_path(vec_ref:&Vec<u64>, vec_sess:&Vec<u64>)-> i32{
    let mut diff: i32 = 0;
    for (idx,ele) in vec_sess.iter().enumerate(){
        if ele != &vec_ref[idx]{
            let diff_value = vec_sess.len() - idx;
            diff = diff_value as i32;
            break;
        }        
    }
    return diff;
}