use std::process::{Command, Stdio};
use std::{fmt::Write, fs, num::ParseIntError, str, sync::Arc};

use anyhow::{Context, Result};
use parking_lot::Mutex;
use qemu_rs::{Address, USize};
use rune::Module;
use std::net::{SocketAddr, UdpSocket};

use lazy_static::lazy_static;

use pyo3::prelude::*; // Import PyO3 functionality
use pyo3::sync::GILOnceCell;
use pyo3::types::PyBytes; // PyBytes to handle byte strings

use super::memory_write;
use crate::hooks::Symbolizer;

lazy_static! {
    static ref queue_string: Mutex<String> = Mutex::new(String::new());
    static ref adv_rpl_data: Mutex<String> = Mutex::new(String::new());
    static ref tx_data: Mutex<String> = Mutex::new(String::new());
    static ref empty_pdu_data: Mutex<String> = Mutex::new(String::new());
    static ref pdu_data: Mutex<String> = Mutex::new(String::new());
    static ref flag2: Mutex<String> = Mutex::new(String::from("0"));
}

pub fn module(symbolizer: Arc<Mutex<Symbolizer>>) -> Result<Module> {
    let mut module = Module::with_crate("common");

    // convert to unsigned
    module.function(&["byte"], byte)?;

    // patch functions
    module.function(&["patch"], |address, bytes| {
        log::warn!("`common::patch(0x{:08x?}, {:02x?})` is deprecated, use `common::patch_address(0x{:08x?}, {:02x?})` instead", address, bytes, address, bytes);
        patch_address(address, bytes)
    })?;
    module.function(&["patch_address"], patch_address)?;
    module.function(&["patch_function"], move |symbol, bytes| {
        patch_function(symbolizer.clone(), symbol, bytes)
    })?;

    module.function(&["system"], system)?;
    module.function(&["decode_hex"], decode_hex)?;
    module.function(&["encode_hex"], encode_hex)?;
    module.function(&["udp_socket"], udp_socket)?;
    module.function(&["running_socket_background"], running_socket_background)?;
    module.function(&["send_socket_data"], send_socket_data)?;
    module.function(&["get_socket_data"], get_socket_data)?;
    module.function(&["get_adv_rpl_data"], get_adv_rpl_data)?;
    module.function(&["update_tx_data"], update_tx_data)?;
    module.function(&["get_empty_pdu_data"], get_empty_pdu_data)?;
    module.function(&["get_data_rpl_data"], get_data_rpl_data)?;
    module.function(&["parse_packet"], parse_packet)?;

    Ok(module)
}

/**
 * @Article Script Functions
 * ## Extract Byte
 * `common::byte(value: usize, n: usize)`
 *
 * Example:
 * ```rune
 * let val = memory::read_u32(addr)?;
 * for n in 0..4 {
 *     println!("{:02x}", common::byte(val, n))
 * }
 * ```
 */
fn byte(value: usize, n: usize) -> USize {
    ((value >> (n * 8)) & 0xff) as USize
}

/**
 * @Article Script Functions
 * ## Patch Binary
 * `common::patch_address(address: usize, bytes: Vec<u8>)`
 * `common::patch_function(symbol: &str, bytes: Vec<u8>)`
 *
 * Patch `bytes` directly at `address` or at all addresses `symbol` resolves to.
 *
 * Example:
 * ```rune
 * common::patch_address(0xdead_beef, arm::RETURN);
 * common::patch_function("some_function", arm::RETURN);
 * ```
 */
fn patch_address(address: Address, bytes: Vec<USize>) -> Result<()> {
    log::debug!("patch: address = {:08x}, bytes = {:02x?}", address, bytes);

    for (i, byte) in bytes.iter().enumerate() {
        let address = address
            .checked_add(i as Address)
            .context("Address overflow")?;

        memory_write::<u8, 1>(
            address,
            (*byte)
                .try_into()
                .with_context(|| format!("Byte {byte:#x?} too large"))?,
        )?;
    }

    Ok(())
}
fn patch_function(
    symbolizer: Arc<Mutex<Symbolizer>>,
    symbol: &str,
    bytes: Vec<USize>,
) -> Result<()> {
    log::debug!("patch: symbol = {:?}, bytes = {:02x?}", symbol, bytes);

    let addresses: Vec<_> = symbolizer
        .lock()
        .resolve_symbol_with_offset(symbol, 0)
        .map(|result| result.with_context(|| format!("Failed resolve symbol {symbol:?}")))
        // .map(|result| result.with_context(|| format!("Failed resolve symbol {symbol:?} with offset {offset:08x}")))
        .collect::<Result<Vec<_>>>()?;
    if addresses.is_empty() {
        anyhow::bail!("Failed to patch: Symbol {:?} not found.", symbol);
    }

    log::info!(
        "Patch function {:?} at {:08x?} with {:02x?}",
        symbol,
        addresses,
        bytes
    );

    for address in addresses {
        patch_address(address, bytes.clone())?;
    }

    Ok(())
}

fn udp_socket() {
    // Bind the socket to a local address and port
    let socket = UdpSocket::bind("127.0.0.1:9999").unwrap();
    loop {
        let mut all_bytes: Vec<u8> = Vec::new();

        let mut buf = [0; 1024]; // Buffer for receiving data
                                 // Receive a message from any source
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        all_bytes.extend_from_slice(&buf[..amt]);

        let hex_string = String::from_utf8(all_bytes).unwrap();

        // println!("uhuuuuuu: {hex_string}");

        // for v in &all_bytes {
        //     print!("{v:02X}");
        // }

        // println!("");

        // Convert the received byte array to a hex string
        // let hex_string: String = all_bytes
        //     .iter()
        //     .map(|byte| format!("0x{:02X}", byte)) // Convert each byte to hex
        //     .collect::<Vec<String>>()
        //     .join(" "); // Join with spaces for readability
        if hex_string == "0x6B 0x69 0x6C 0x6C 0x20 0x73 0x6F 0x63 0x6B 0x65 0x74" {
            println!("Received kill signal");
            break;
        } else {
            // state.receive_pkt = hex_string;
            // println!("Received {} bytes from {}: {}", amt, src, hex_string);
            // rcv_pkt = &hex_string;
            // println!("This is rcv_pkt {}", rcv_pkt);
            // let hex = LazyLock::force(queue_string);

            // let mut str_g = queue_string.lock();
            queue_string.lock().clear();
            queue_string.lock().push_str(&hex_string);
            // str_g.clear();
            // str_g.push_str(&hex_string);
            println!("Updated queue string {}", hex_string);
            // let mut hex = &*queue_string;
            // hex.push_str("sdsds");
            //  = hex_string;
            // You can also send a reply to the source if needed
            let reply = b"Message received!";
            socket.send_to(reply, src).unwrap();
        }
    }
    // Ok(())
}

fn running_socket_background() {

    // let udp_thread = thread::spawn(move || {
    //     udp_socket(rcvd_pkt);
    // });
}

fn send_socket_data(msg: String) {
    let socket = UdpSocket::bind("127.0.0.1:8888").unwrap();
    let remote_addr: SocketAddr = "127.0.0.1:7777".parse().unwrap();
    socket.send_to(msg.as_bytes(), remote_addr).unwrap();
    println!("Sent message from peripheral to {}: {:?}", remote_addr, msg);
}

fn get_socket_data() -> String {
    let x = queue_string.lock().clone();
    return x;
}

/// Executes a system command and returns its output as a `String`.
///
/// This function takes a `String` representing the command to execute, and returns the output of that command as a `String`. It uses the `Command` struct from the standard library to execute the command, capture its standard output, and return it as a `String`.
///
/// # Arguments
/// * `cmd` - A `String` representing the command to execute.
///
/// # Returns
/// A `String` containing the output of the executed command.
fn system(cmd: String) -> String {
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        .unwrap();

    let ret = String::from_utf8(output.stdout).unwrap();

    if ret.ends_with("\n") {
        return ret[0..ret.len() - 1].to_string();
    }

    return ret;
}

fn decode_hex(s: &str) -> Result<Vec<u32>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u32::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn encode_hex(bytes: Vec<u32>) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        _ = write!(&mut s, "{:02x}", b);
    }

    return s;
}

fn update_tx_data(tx: String) {
    tx_data.lock().clear();
    tx_data.lock().push_str(&tx);
}

fn generate_adv_rpl() -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python script (ensure it's in the same directory or in PYTHONPATH)
        let module = bt_module(py);

        // Prepare the byte data you want to pass to the `handle_data` function
        let handle_data = module.getattr("handle_adv")?;
        let result = handle_data.call1((PyBytes::new_bound(py, get_tx_data().as_bytes()),))?;
        let result_str = std::str::from_utf8(result.extract()?)?;
        let owned_string = result_str.to_string();

        // Update global varible for rx data retrival
        adv_rpl_data.lock().clear();
        adv_rpl_data.lock().push_str(&owned_string);
        Ok(())
    })
}

fn generate_empty_pdu_rpl() -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python script (ensure it's in the same directory or in PYTHONPATH)
        let module = bt_module(py);

        // Prepare the byte data you want to pass to the `handle_data` function
        // let py_byte_data = PyBytes::new_bound(py,inputdata);
        let handle_data = module.getattr("generate_empty_pdu")?;
        // Call the 'handle_data' function with the byte data
        // let result = module.call1("handle_data", (PyBytes::new(py, input_data),))?;
        // let result = handle_data.call1((py_byte_data,))?;
        let result = handle_data.call0()?;
        // let value: String = result.extract::<String>()?;
        let result_bytes: &[u8] = result.extract()?;
        // let mut s = String::with_capacity(result_bytes.len() * 2);
        // for b in result_bytes {
        //     _ = write!(&mut s, "{:02x}", b);
        // }

        // return s;
        // let result_as_hex = hex::encode(result_bytes);
        // println!("Result from Python: {:?}", result);
        let result_str = std::str::from_utf8(result_bytes).unwrap();
        let owned_string = result_str.to_string();
        empty_pdu_data.lock().clear();
        empty_pdu_data.lock().push_str(&owned_string);
        // println!("Result from Python:  {:?}", result_bytes);
        // println!("Result from Python: b'{}'", owned_string);
        // Extract the result as a PyBytes (assuming the return value is also a byte string)
        // return owned_string;
        Ok(())
    })
}

static PY_MODULE: GILOnceCell<Py<PyModule>> = GILOnceCell::new();

fn bt_module(py: Python<'_>) -> &Bound<'_, PyModule> {
    PY_MODULE
        .get_or_init(py, || {
            PyModule::from_code_bound(
                py,
                fs::read_to_string("scripts/ble_stack.py").unwrap().as_str(),
                "ble_stack.py",
                "ble_stack",
            )
            .unwrap()
            .unbind()
        })
        .bind(py)
}

const PROTO_BLE: &str = "ble";
const SUPPORTED_PROTOCOLS: &[&str] = &[PROTO_BLE];

fn parse_packet(
    proto_name: String,
    packet: String,
    direction: usize,
    channel_mode: bool,
    show_pkt: bool,
) -> String {
    Python::with_gil(|py| -> Result<String, String> {
        if !SUPPORTED_PROTOCOLS.contains(&proto_name.as_str()) {
            return Err(format!("Protocol Name not found: {proto_name}"));
        }

        let parse_fcn = match proto_name.as_str() {
            PROTO_BLE => {
                let module = bt_module(py);
                Ok(module.getattr("parse_ble_packet").unwrap())
            }
            _ => Err("Protocol Function not found: parse_ble_packet"),
        }?;

        Ok(parse_fcn
            .call1((packet, direction, channel_mode, show_pkt))
            .unwrap()
            .extract()
            .unwrap())
    })
    .unwrap()
}

fn generate_data_rpl() -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python script (ensure it's in the same directory or in PYTHONPATH)
        let module = bt_module(py);

        // Prepare the byte data you want to pass to the `handle_data` function
        let input_data = get_tx_data();
        // println!("This is input data: {}",input_data);
        let py_byte_data = PyBytes::new_bound(py, input_data.as_bytes());
        let handle_data = module.getattr("handle_data")?;
        // Call the 'handle_data' function with the byte data
        // let result = module.call1("handle_data", (PyBytes::new(py, input_data),))?;
        let flag_want = flag2.lock().clone();

        // println!("This is flag want: {}",flag_want);

        let result = handle_data.call1((py_byte_data, flag_want))?;
        if flag2.lock().clone() == "0" {
            flag2.lock().clear();
            flag2.lock().push_str("1");
        } else {
            flag2.lock().clear();
            flag2.lock().push_str("0");
        }

        // Extract the result as a PyBytes (assuming the return value is also a byte string)
        let bytes_string =  std::str::from_utf8(result.extract()?)?;
        pdu_data.lock().clear();
        pdu_data.lock().push_str(&bytes_string);
        Ok(())
    })
}

fn get_adv_rpl_data() -> String {
    _ = generate_adv_rpl();
    adv_rpl_data.lock().clone()
}

fn get_data_rpl_data() -> String {
    _ = generate_data_rpl();
    pdu_data.lock().clone()
}

fn get_tx_data() -> String {
    tx_data.lock().clone()
}

fn get_empty_pdu_data() -> String {
    _ = generate_empty_pdu_rpl();
    empty_pdu_data.lock().clone()
}
