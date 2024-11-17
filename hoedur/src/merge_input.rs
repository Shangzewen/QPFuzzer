use anyhow::{Context, Result};
use modeling::input::InputFile;
use std::path::Path;

fn main() -> Result<()> {
    // Replace "path_to_input1.bin" and "path_to_input2.bin" with actual file paths
    let input_path1 = Path::new("path_to_input1.bin");
    let input_path2 = Path::new("path_to_input2.bin");

    // Read the input files
    let input_file1 = InputFile::read_from_path(input_path1)?;
    let input_file2 = InputFile::read_from_path(input_path2)?;

    // Merge input_file2 into input_file1
    let merged_input_file = input_file1.merge(input_file2);

    // Save or display the result
    println!("Merged Input File: {:?}", merged_input_file);

    // Write merged input to a new file, if needed
    let output_path = Path::new("merged_input_file.bin");
    merged_input_file.write_to(std::fs::File::create(output_path)?)?;

    Ok(())
}