use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};

fn parse_file() -> Result<Vec<String>, Error> {
    let mut v = Vec::new();
    let file_path = Path::new("./input.txt");

    let file = File::open(file_path)?;
    let buf = BufReader::new(file);

    for i in buf.lines() {
        v.push(i?);
    }
    Ok(v)
}


// Idea
// Convert the puzzle input into a 2D matrix
// then find all adjacent elements

fn get_nearby(v: Vec<char>, index: usize) {}

fn find_adjacents(v: Vec<[char; 10]>) {
    let mut s = 0;
    let mut already_searched: Vec<usize> = Vec::new();
    // let mut adjacent_vec = Vec::new();


    for (index, row) in v.iter().enumerate() {
        for (index_of_row_elem, row_elem) in row.iter().enumerate() {
            if *row_elem == '@' {
                already_searched.push(index_of_row_elem);
                if index == 0 {
                    // only search the bottom not the top
                    let adj1 = get_nearby(v[index + 1].to_vec(), index_of_row_elem);
                    
                }
            }
        }
    }
}

fn main() {
    let test_inp = vec![
        ['.', '.', '@', '@', '.', '@', '@', '@', '@', '.'],
        ['@', '@', '@', '.', '@', '.', '@', '.', '@', '@'],
        ['@', '@', '@', '@', '@', '.', '@', '.', '@', '@'],
        ['@', '.', '@', '@', '@', '@', '.', '.', '@', '.'],
        ['@', '@', '.', '@', '@', '@', '@', '.', '@', '@'],
        ['.', '@', '@', '@', '@', '@', '@', '@', '.', '@'],
        ['.', '@', '.', '@', '.', '@', '.', '@', '@', '@'],
        ['@', '.', '@', '@', '@', '.', '@', '@', '@', '@'],
        ['.', '@', '@', '@', '@', '@', '@', '@', '@', '.'],
        ['@', '.', '@', '.', '@', '@', '@', '.', '@', '.'],
    ];
    println!("Hello, world!");
}
