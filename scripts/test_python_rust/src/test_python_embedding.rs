use pyo3::prelude::*; // Import PyO3 functionality
use pyo3::types::PyBytes; // PyBytes to handle byte strings

fn main() -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python script (ensure it's in the same directory or in PYTHONPATH)
        let module = py.import("my_script")?; // Change 'my_script' to your actual Python file name without extension

        // Prepare the byte data you want to pass to the `handle_data` function
        let input_data = b"60230000000000c002010607030d180f1805181107f0debc9a785634127856341278563412";

        // Call the 'handle_data' function with the byte data
        let result = module.call1("handle_data", (PyBytes::new(py, input_data),))?;

        // Extract the result as a PyBytes (assuming the return value is also a byte string)
        let byte_result: &PyBytes = result.extract()?;

        // Convert PyBytes to a Rust byte slice
        let bytes: &[u8] = byte_result.as_bytes();

        // Print the byte string (you can also convert it to a Rust String if it's UTF-8)
        println!("{:?}", bytes); // Print the raw byte result

        // Optionally, if the byte string is valid UTF-8, you can convert it to a Rust string:
        match std::str::from_utf8(bytes) {
            Ok(text) => println!("Decoded Text: {}", text), // Prints if it can be decoded
            Err(_) => println!("Non-UTF-8 Byte Data: {:?}", bytes), // Print raw bytes if it's not valid UTF-8
        }

        Ok(())
    })
}