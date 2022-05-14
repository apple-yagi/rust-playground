use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::prelude::*;

fn main() {
    let filename = "Cargo.toml";

    // Open file
    let mut file = fs::File::open(filename).expect("Failed to open file");

    // Print file size
    println!("{}", file.metadata().unwrap().len());

    // Read file
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Gzip encode
    let output = Vec::new();
    let mut encoder = GzEncoder::new(output, Compression::default());
    encoder
        .write_all(contents.as_bytes())
        .expect("Error: write_all(contents.as_bytes())");
    let output = encoder.finish().unwrap();
    println!("{:?}", output.len());
}
