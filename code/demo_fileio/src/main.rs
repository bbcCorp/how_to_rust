// demo of file IO
use std::fs::File;
use std::io::Error;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    println!("=== Demo of file operations === ");

    let read_from_file = Path::new("data.txt");
    let try_read = reading_file(&read_from_file);

    match try_read {
        Ok(data) => println!("File data: {}", data),
        Err(err) => print!("Error reading file content: {:?}\n\n", err),
    }
}

fn reading_file(file_path: &Path) -> Result<String, Error> {
    // we will read into this buffer
    let mut data_buffer = String::new();

    let mut fp = File::open(file_path)?;
    fp.read_to_string(&mut data_buffer)?;

    Ok(data_buffer)
}
