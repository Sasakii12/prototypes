use std::fs::File;
use std::io::{self,Result, Error, BufRead, BufReader};
use std::path::Path;


#[derive(Debug, Copy, Clone)]
enum Rotation {
    L(u32),
    R(u32)
}

fn parse_dial(s: String) -> Rotation {
    let a = s.split_at(1);
    let dial = match a.0 {
        "L" => Rotation::L(a.1.parse().unwrap()),
        "R" => Rotation::R(a.1.parse().unwrap()),
        _ => panic!("Dial not formatted correctly")
    };
    dial
}

fn rotate_dial(dial: u32,rot : Rotation) -> i32 {
    match rot {
        Rotation::L(num) => {
            let temp = dial as i32 - num as i32;
            if temp < 0 {
                temp.rem_euclid(100) 
            } else {
                temp
            }
        }
        Rotation::R(num) => {
            let temp = dial as i32 + num as i32 ;
            if temp > 99 {
                temp % 100
            } else {
                temp
            }
        }
    }
}
// -> 
fn parse_file() -> Result<Vec<String>> {
    let mut rots = Vec::new();
    let file_path = Path::new("./input.txt");

    let file = File::open(file_path)?;

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        rots.push(line.unwrap());
    }
    Ok(rots)
}

fn count_zeros(v : Vec<String>, curr_dial: u32) -> u32 {
    let mut dial = curr_dial;
    let mut zeros = 0;
    for rots in v {
        let rot = parse_dial(rots);
        let n = rotate_dial(dial, rot);
        dial = n as u32;
        if dial == 0 {
            zeros += 1;
        }
        println!("Rotation: {:?}, Dial: {}", rot, dial)
    }
    zeros
}

fn rotate_dial_cycles(dial: u32,rot : Rotation) -> i32 {
    match rot {
        Rotation::L(num) => {
            let temp = dial as i32 - num as i32;
            if temp < 0 {
                temp / 100
            } else {
                0
            }
        }
        Rotation::R(num) => {
            let temp = dial as i32 + num as i32 ;
            if temp > 99 {
                temp / 100
            } else {
                0
            }
        }
    }
}

fn count_dial_zero(v : Vec<String>, curr_dial: u32) -> i32 {
    let mut dial = curr_dial;
    let mut zeros = 0;
    for rots in v {
        let rot = parse_dial(rots);
        let n = rotate_dial_cycles(dial, rot);
        dial = n as u32;
        if dial == 0 {
            zeros += 1;
        }
        println!("Rotation: {:?}, Dial: {}", rot, dial)
    }
    zeros
}

fn main() {
    // let mut zeros = 0;
    // let mut current_dial = 50;
    let temp_inp = vec!["L68","L30","R48","L5","R60","L55","L1","L99","R14","L82"];
    // let p = parse_dial("L51".to_string());
    // println!("{:?}", rotate_dial(50, p));

    // for i in temp_inp {
    //     println!("{}", i);
    //     let rot = parse_dial(String::from(i));
    //     let n = rotate_dial(current_dial, rot);
    //     println!("{}", n);
    //     current_dial = n.try_into().unwrap();
    //     if n == 0 {
    //         zeros += 1
    //     }
    // }
    let n = parse_file().unwrap();
    // temp_inp.into_iter().map(String::from).collect()
    println!("{}", count_zeros(temp_inp.into_iter().map(String::from).collect(), 50))
}
