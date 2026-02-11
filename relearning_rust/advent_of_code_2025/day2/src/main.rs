use std::fs::File;
use std::io::{self,Result, Error, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn parse_file() -> Result<Vec<String>> {
    let mut prod_ids = Vec::new();
    let file_path = Path::new("./input.txt");

    let file = File::open(file_path)?;

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        prod_ids.push(line.unwrap().split(",").collect());
    }
    Ok(prod_ids)
}

fn parse_input() {
    let re = Regex::new(r"^(\d+)\1$").unwrap();
}


fn main() {
    println!("Hello, world!");
    let prod_ids = parse_file().unwrap();
    println!("{:?}", prod_ids)
}
