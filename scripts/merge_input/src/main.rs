use anyhow::{Result};
use modeling::input::{InputFile};
use modeling::hardware::{WriteTo};
use std::path::Path;

// In order to write to a new output file it requires sudo previledge
// Use this command
// sudo env "PATH=$PATH" cargo run
// Try explicitly passing the environment variables using env to preserve the PATH as it is in your regular user session:
// This command tells sudo to use the same PATH as your current shell session, where cargo is found.
fn main() -> Result<()> {
    // Replace "path_to_input1.bin" and "path_to_input2.bin" with actual file paths
    let input_path1 = Path::new("~/qpfuzzer/target-zephyr/runs/input/input-80526.bin");
    let input_path2 = Path::new("~/qpfuzzer/target-zephyr/runs/input/input-24780.bin");

    // Read the input files
    let input_file1 = InputFile::read_from_path(input_path1)?;
    let input_file2 = InputFile::read_from_path(input_path2)?;

    // Merge input_file2 into input_file1
    let merged_input_file = input_file1.merge(input_file2);

    // Save or display the result
    println!("Merged Input File: {:?}", merged_input_file);

    // Write merged input to a new file, if needed
    let output_path = Path::new("~/qpfuzzer/target-zephyr/runs/input/merged_input_file.bin");
    merged_input_file.write_to(std::fs::File::create(output_path)?)?;

    Ok(())
}
