use std::fs::File;
use std::ops::Index;
use std::path::Path;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

use itertools::Itertools;

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
    let mut arr_c = arr.clone();

    let max1 = *arr_c.iter().max().unwrap();
    let combo = arr.iter()
        .combinations(2)
        .filter(|x| *x[0] == max1 || *x[1] == max1)
        .collect::<Vec<Vec<&u32>>>();
    let max_companion = combo.iter()
        .map(|pair| {
            if *pair[0] == max1 { *pair[1] } else { *pair[0] }
        })
        .max()
        .unwrap();
    println!("{:?}" ,max_companion);
    todo!()
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
