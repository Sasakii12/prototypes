use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


#[derive(Debug)]
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
                100 + temp
            } else {
                temp
            }
        }
        Rotation::R(num) => {
            let temp = dial as i32 + num as i32 ;
            if temp > 99 {
                temp - 100 
            } else {
                temp
            }
        }
    }
}

fn parse_file() -> io::Result<()> {
    let file_path = Path::new("input.txt");

    let file = File::open(file_path)?;

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        
    }
}

fn main() {
    let mut zeros = 0;
    let mut current_dial = 50;
    let temp_inp = vec!["L68","L30","R48","L5","R60","L55","L1","L99","R14","L82"];
    let p = parse_dial("L51".to_string());
    println!("{:?}", rotate_dial(50, p));

    for i in temp_inp {
        println!("{}", i);
        let rot = parse_dial(String::from(i));
        let n = rotate_dial(current_dial, rot);
        println!("{}", n);
        current_dial = n.try_into().unwrap();
        if n == 0 {
            zeros += 1
        }
    }
    println!("{}", zeros)
}
