use std::fmt::format;
use hex;
fn update_payload_len(hex_str: &str, header_base: &str)-> String{
    let link_data_len = (hex_str.len()/2);
    // println!("This is the link len: {}",link_data_len);
    let payload_len = link_data_len + 17;
    // println!("This is the payload len: {}",payload_len);
    let hex_payload_len = format!("{:x}",payload_len);
    // println!("This is the hex payload len: {}",link_data_len);
    let updated_hex = format!("{}{}{}", 
        &header_base[0..2],         // Keep "06"
        hex_payload_len,             // Replace with calculated length
        &header_base[4..],          // Keep rest
    );
    updated_hex
    // println!("This is the updated header: {}",updated_hex);
}
fn update_channel_num(pkt_type: usize, header_base: &str) -> String {
    let channel_adv = "27";   // Advertising channel
    let channel_data = "03";   // Data channel
    let channel_offset = 18;  // Byte index where channel is located

    if header_base.len() < channel_offset + 2 {
        panic!("Header too short to update channel field.");
    }

    let channel_value = if pkt_type == 0 { channel_adv } else { channel_data };
    // println!("{}",&header_base[0..channel_offset]);
    format!(
        "{}{}{}",
        &header_base[0..channel_offset],
        channel_value,
        &header_base[channel_offset + 2..]
    )
}
fn verify_payload(payload: &str) -> String {
    // Extract the length byte and parse it as hex
    let pdu_len_byte = &payload[2..4];
    let expected_len = usize::from_str_radix(pdu_len_byte, 16).unwrap();

    // Actual payload length in bytes (payload is hex, so divide by 2), minus 2 for header
    let actual_len = (payload.len() / 2) - 2;

    if actual_len == expected_len {
        payload.to_string()
    } else if actual_len < expected_len {
        // Add missing "00" bytes
        let missing_bytes = expected_len - actual_len;
        let mut padded_payload = payload.to_string();
        padded_payload.push_str(&"00".repeat(missing_bytes));
        padded_payload
    } else {
        // Optional: truncate if actual_len > expected_len?
        // Otherwise just return original
        payload.to_string()
    }
}

fn construct_pkt(header_base: &str, ble_ll_data: &str, crc: &str, event: &str, acc_addr_adv: &str, acc_addr_data: &str, pkt_type:usize)-> String{
    // 0 pkt_type means adv channel 1 means data channel

    if (pkt_type == 0){
        let update_payload = verify_payload(ble_ll_data);
        let update_header_pay_len = update_payload_len(&update_payload, header_base);
        let update_header = update_channel_num(pkt_type, &update_header_pay_len);
        let constructed_pkt = format!("{}{}{}{}{}",event,update_header,acc_addr_adv,update_payload,crc);
        constructed_pkt
    }else{
        // remove the extra lengh byte "00" for data pdu
        let ble_ll_data_clean =format!(
        "{}{}",
        &ble_ll_data[0..4],
        &ble_ll_data[6..]
        );
        let update_payload = verify_payload(&ble_ll_data_clean);
        let update_header_pay_len = update_payload_len(&update_payload, header_base);
        let update_header = update_channel_num(pkt_type, &update_header_pay_len);
        let constructed_pkt = format!("{}{}{}{}{}",event,update_header,acc_addr_data,update_payload,crc);
        constructed_pkt
    }

}
fn main() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    // Request socket will always send request first and expect a reply before it send another request
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://127.0.0.1:5555").is_ok());

    let mut msg = zmq::Message::new();
    //  wdissector pkt structure evt_byte + 

    for request_nbr in 0..10 {
        let hex_str = "830cf37a7d65de280000000000c0";
        let access_adr_adv = "d6be898e";
        let access_adr_data = "7083329a";
        let crc = "000000";
        let header_base = "061c0002c06a060a01031d00005ef50000";
        let event = "00";
        // let pkt_str = format!("{}{}{}{}{}",event,header_base,access_adr_data,hex_str,crc);
        let pkt_str = construct_pkt(header_base, hex_str, crc, event, access_adr_adv, access_adr_data, 0);
        // update_pkt_str = 
        println!("{}",pkt_str);
        // update_payload_len(hex_str, "06190002c06a060a01031d00005ef50000");
        // let updated_header = update_channel_num(0, header_base);
        // let updated_payload = verify_payload(hex_str);
        // println!("{}",updated_header);
        // println!("{}",updated_payload);
        let raw_bytes = hex::decode(pkt_str).expect("Invalid hex string");
        println!("Sending BLE PKT {}...", request_nbr);
        requester.send(raw_bytes.as_slice(), 0).unwrap();
        // the 0 means the mode is block mode, will wait until have response
        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
        
    }
}
