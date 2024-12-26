use std::process::{Command, Stdio};
use std::{fmt::Write, fs, num::ParseIntError, str, sync::Arc};
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, BufRead, BufReader, Write as IoWrite};
use std::thread;
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
const SOCKET_PATH: &str = "/tmp/central.sock";
lazy_static! {
    static ref rx_data: Mutex<String> = Mutex::new(String::new());
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
    // module.function(&["udp_socket"], udp_socket)?;
    module.function(&["running_socket_background"], running_socket_background)?;
    module.function(&["get_rx_data"], get_rx_data)?;
    module.function(&["send_socket_data"], send_socket_data)?;
    module.function(&["get_socket_data"], get_socket_data)?;
    module.function(&["get_adv_rpl_data"], get_adv_rpl_data)?;
    module.function(&["update_tx_data"], update_tx_data)?;
    module.function(&["clear_tx_data"],clear_tx_data)?;
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

fn parse_hex_to_u8_array(input: &str) -> Vec<u8> {
    input
        .split_whitespace() // Split the input string by spaces
        .map(|hex| u8::from_str_radix(hex, 16).expect("Invalid hex number")) // Convert each hex string to u8
        .collect() // Collect into a Vec<u8>
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
            // let input = "60 23 00 00 00 00 00 c0 02 01 06 07 03 0d 18 0f 18 05 18 11 07 f0 de bc 9a 78 56 34 12 78 56 34 12 78 56 34";
            let input = &format_hex_string(&get_tx_data());
            println!("input: {}", input);
            let response = parse_hex_to_u8_array(input);
            stream.write_all(&response)?;
            println!("Sent: {:?}",response);
        } else {
            // Default response
            let response = format!("ACK: {}", buffer.trim_end());
            update_rx_data((buffer.trim_end()).to_string());
            stream.write_all(response.as_bytes())?;
        }
        stream.flush()?;
    }

    Ok(())
}

fn unix_socket() -> io::Result<()> {
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

fn running_socket_background(){
    thread::spawn(move || {
        if let Err(err) = unix_socket() {
            eprintln!("Unix socket error: {}", err);
        }
     });
}

fn send_socket_data(msg: String) {
    let socket = UdpSocket::bind("127.0.0.1:8888").unwrap();
    let remote_addr: SocketAddr = "127.0.0.1:7777".parse().unwrap();
    socket.send_to(msg.as_bytes(), remote_addr).unwrap();
    println!("Sent message from peripheral to {}: {:?}", remote_addr, msg);
}

fn get_socket_data() -> String {
    let x = rx_data.lock().clone();
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
        // let testttt = get_tx_data();
        // let input_test =format_hex_string(&testttt); 
        // println!("This is get_tx_data: {}", input_test);
        // Prepare the byte data you want to pass to the `handle_data` function
        let handle_data = module.getattr("handle_adv")?;
        let result = handle_data.call1((PyBytes::new_bound(py, get_tx_data().as_bytes()),))?;
        let result_str = std::str::from_utf8(result.extract()?)?;
        let owned_string = result_str.to_string();
        // println!("This is owned_string: {}",owned_string);

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
fn get_rx_data() -> String {
    rx_data.lock().clone()
}
fn clear_tx_data(){
    tx_data.lock().clear();
    // println!("This is tx_data_buffer: {}",tx_data.lock().clone());
    tx_data.lock().push_str("010000");
    // println!("This is tx_data_buffer_after_push: {}",tx_data.lock().clone());
    log::info!("Start from beagining, update tx_buffer to empty_pdu")
}

fn get_empty_pdu_data() -> String {
    _ = generate_empty_pdu_rpl();
    empty_pdu_data.lock().clone()
}
fn update_rx_data(rx: String) {
    rx_data.lock().clear();
    rx_data.lock().push_str(&rx);
}
