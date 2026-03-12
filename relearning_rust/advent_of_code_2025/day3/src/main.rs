use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Result};

fn read_input(file: &str) -> Result<Vec<String>> {
    let mut output = Vec::new();
    let file_path = Path::new(file);

    let fs = File::open(file_path)?;

    let buf = BufReader::new(fs);

    for lines in buf.lines() {
        output.push(lines.unwrap());
    }
    Ok(output)
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", read_input("test.txt").unwrap())

}
