use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};


fn open_file() {
    let f = File::open("input.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("input.txt").unwrap_or_else(|error| {
                panic!("Failure creating file");
            })
        } else {
            panic!("Error opening the file")
        }
    });
}

fn read_file() -> Result<String, io::Error> {
    let file = File::open("input.txt")?;
    let mut str = String::new();
    let buf_reader = BufReader::new(file);

    for i in buf_reader.lines() {
        str.push_str(&i.unwrap());
        str.push(' ');
    }

    Ok(str)
}

fn main() {
    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 1;        
        println!("{}", i)
    };
    println!("{}",read_file().unwrap())
}
