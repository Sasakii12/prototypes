use std::fs::File;
use std::ops::Index;
use std::path::Path;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

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

fn find_max(arr: &Vec<u32>) -> HashMap<u32, u32>  {
    let mut arr_clone = arr.clone();
    arr_clone.sort();
    let mut n = Vec::new();
    let mut hash: HashMap<u32, u32> = HashMap::new();


    for _ in 0..2 {
        n.push(arr_clone.pop().unwrap());
    }
    for i in n {
        hash.insert(arr.iter().position(|r| *r == i).unwrap() as u32, i);
    }
    hash
}

fn string_to_int_vec(arr: &String) -> Vec<u32> {
    arr.chars().filter_map(|char| char.to_digit(10)).collect()
}

fn sum_of_jolt(batteries: Vec<String>) {
    let battery_nums = batteries.iter().map(string_to_int_vec).collect::<Vec<Vec<u32>>>();
    let jolts = battery_nums.iter().map(find_max)
    .map(|x| {
        let mut v = Vec::new();
        let mut keys = x.keys().collect::<Vec<&u32>>();
        keys.sort();
        for i in keys {
            v.push(x.get(i).unwrap().to_owned());
        }
        v
    }).collect::<Vec<Vec<u32>>>();
    println!("{:?}", jolts)
}

fn main() {
    println!("Hello, world!");
    let inp = read_input("test.txt").unwrap();
    let mut arr = string_to_int_vec(&inp[0]);
    // arr.sort();
    println!("{:?}", arr);
    sum_of_jolt(inp);
    // println!("{:?}", find_max(&arr).get(&1).unwrap())

}
