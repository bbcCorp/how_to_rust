// demo of file IO
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    println!("=== Demo of file operations === ");

    // Demo how to read from a file
    let read_from_file = Path::new("data.txt");
    let try_read = read_file(&read_from_file);

    match try_read {
        Ok(data) => println!("File data: {}", data),
        Err(err) => print!("Error reading file content: {:?}\n\n", err),
    }

    // Demo - Buffered reading
    let try_buf_read = read_file_buffered(&read_from_file);

    match try_buf_read {
        Ok(data) => println!("File data: {}", data),
        Err(err) => print!("Error reading file content: {:?}\n\n", err),
    }

    // Demo: How to write to a file
    let write_to_filename = Path::new("rust-test-fileio.txt");
    let data = String::from("A journey of a thousand miles begins with a single step");
    let try_write = write_to_file(&write_to_filename, &data);

    match try_write {
        Ok(_) => println!("Data written to file successfully"),
        Err(err) => print!("Error writing file content: {:?}\n\n", err),
    }
}

fn read_file(file_path: &Path) -> Result<String, Error> {
    // we will read into this buffer
    let mut data_buffer = String::new();

    let mut fp = File::open(file_path)?;

    fp.read_to_string(&mut data_buffer)?;

    Ok(data_buffer)
}

fn read_file_buffered(file_path: &Path) -> Result<String, Error> {
    // we will read into this buffer
    let mut data_buffer = String::new();

    let fp = File::open(file_path)?;

    // Reference: https://doc.rust-lang.org/std/io/struct.BufReader.html
    // Currently, BufReader reads 8Kib of data into cache, and is way more efficient then direct Reads

    let mut buf_reader: BufReader<File> = BufReader::new(fp);

    let _len = buf_reader.read_to_string(&mut data_buffer)?;

    Ok(data_buffer)
}

fn write_to_file(file_path: &Path, data: &str) -> std::io::Result<()> {
    // create/update the file
    let mut fp = File::create(file_path)?;
    fp.write_all(data.as_bytes())?;

    Ok(())
}
