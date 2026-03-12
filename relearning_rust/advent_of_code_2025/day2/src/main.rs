use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error, Read, Result};
use std::path::Path;
use fancy_regex::Regex;
use itertools::Itertools;

fn parse_file() -> Result<String> {
    let file_path = Path::new("./input.txt");

    let file = fs::read_to_string(file_path)?;
    Ok(file)
}

fn parse_input_into_range(input: &str) -> Vec<(u64, u64)> {
    let formatted_inp = input.split(",")
    .map(|x| x.split("-")
    .map(|r| r.to_string().parse::<u64>().unwrap())
    .collect_tuple::<(u64, u64)>().unwrap())
    .collect::<Vec<(u64,u64)>>();
    formatted_inp
}

fn eval_ranges(input: Vec<(u64, u64)>) -> u64 {
    let mut sum = 0;
    // part 1 regex
    // let re = Regex::new(r"^(\d+)\1$").unwrap();
    
    // part 2 regex
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    for range in input {
        for i in range.0 .. range.1 + 1 {
            if re.is_match(i.to_string().as_str()).unwrap() {
                println!("{}", i);
                sum += i
            }
        }
    }
    sum
}


fn main() {
    let test_string = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    // let prod_ids = parse_file().unwrap();
    let ranges = parse_input_into_range(test_string);
    let file_inp = parse_file().unwrap();
    // println!("{}", file_inp);
    let input_ranges = parse_input_into_range(&file_inp);

    // the compiler took 28 seconds to evaluate all of this LOL
    println!("{}", eval_ranges(input_ranges));
    // println!("{:?}", eval_ranges(ranges.clone()));
    
    // assert!(eval_ranges(ranges) == 4174379265)

}
