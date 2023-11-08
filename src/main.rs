extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Ensure the correct number of arguments are provided.
    if args.len() != 3 {
        eprintln!("Usage: {} <source_file> <target_file>", args[0]);
        std::process::exit(1);
    }

    let source_path = Path::new(&args[1]);
    let target_path = Path::new(&args[2]);

    // Check if the source file exists.
    if !source_path.exists() {
        eprintln!("Error: Source file '{}' not found.", source_path.display());
        std::process::exit(1);
    }

    // Open the source file for reading.
    let source_file = File::open(&source_path)?;
    let reader = BufReader::new(source_file);

    // Create the target file for writing.
    let target_file = File::create(&target_path)?;

    // Create a Gzip encoder with the best compression level.
    let mut encoder = GzEncoder::new(target_file, Compression::best());

    // Start the timer.
    let start = Instant::now();

    // Perform the file compression.
    let bytes_copied = io::copy(&mut reader.by_ref(), &mut encoder)?;

    // Finish the compression and flush the output.
    let output = encoder.finish()?;

    // Print statistics about the compression.
    let input_metadata = source_path.metadata()?;
    let output_metadata = output.metadata()?;

    println!("Source length: {}", input_metadata.len());
    println!("Target length: {}", output_metadata.len());
    println!("Compression ratio: {:.2}%", (output_metadata.len() as f64 / input_metadata.len() as f64) * 100.0);
    println!("Bytes copied: {}", bytes_copied);
    println!("Elapsed time: {:?}", start.elapsed());

    Ok(())
}
