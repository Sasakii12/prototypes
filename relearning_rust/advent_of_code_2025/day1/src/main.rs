use std::fs::File;
use std::io::{self,Result, Error, BufRead, BufReader};
use std::path::Path;

enum Rotation {
    L(u32),
    R(u32),
}

struct Dial {
    dial: u32,
    clicks: u32,
}

impl Dial {

    fn parse_rot(&self,rot: &str) -> Rotation {
        let rot_split = rot.split_at(1);

        match rot_split.0 {
            "L" => Rotation::L(rot_split.1.parse().unwrap()),
            "R" => Rotation::R(rot_split.1.parse().unwrap()),
            _ => panic!("rotation string not formatted correctly")

        }
        
    }

    fn rotation(&mut self, rot: Rotation) {
        match rot {
            Rotation::L(val) => self.rotation_l(val),
            Rotation::R(val) => self.rotation_r(val),
        }
    }

    fn rotation_r(&mut self, num: u32) {
        let val = self.dial + num;
        let val_mod = val.rem_euclid(100);
        let cycles = (val - val_mod) / 100;
        println!("dial: {}, num: {}", self.dial, num);
        println!("cycle: {}, val_mod: {}, val: {}", cycles, val_mod, val);
        if val_mod == 0 {
            self.clicks += 1;
            self.dial = 0;
            // self.clicks += cycles;
        } else {
            self.dial = val_mod;
            self.clicks += cycles;
        }
        println!("total clicks: {}", self.clicks)
        
    }

    fn rotation_l(&mut self, num: u32) {
        let val = self.dial as i32 - num as i32;
        let val_mod = val.rem_euclid(100) as i32;
        let cycles = ((val as i32 - val_mod) / 100).abs() as u32;
        println!("dial: {}, num: {}", self.dial, num);
        println!("cycle: {}, val_mod: {}, val: {}", cycles, val_mod, val);
        if val_mod == 0 {
            self.clicks += 1;
            self.dial = 0;
            // self.clicks += cycles;
        } else {
            if self.dial != 0 {
                self. clicks += cycles;
            }
            self.dial = val_mod as u32;
            
        }
        println!("total clicks: {}", self.clicks)
    }
}

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

fn main() {
    let temp_inp = vec!["L68","L30","R48","L5","R60","L55","L1","L99","R14","L82"];
    let puzzle_input = parse_file().unwrap();
    let mut dial = Dial {dial : 50, clicks : 0};
    for i in temp_inp {
        let rot = dial.parse_rot(&i);
        dial.rotation(rot);
        
    }
    println!("dial: {}, clicks: {}", dial.dial, dial.clicks)
}