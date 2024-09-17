use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{fmt::Write, num::ParseIntError};

use anyhow::{Context, Result};
use parking_lot::Mutex;
use qemu_rs::{Address, USize};
use rune::Module;

use crate::hooks::Symbolizer;

use super::memory_write;

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
