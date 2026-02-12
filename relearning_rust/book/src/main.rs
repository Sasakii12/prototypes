use std::fs::File;
use std::io::ErrorKind;


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

fn main() {
    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 1;        
        println!("{}", i)
    }
    open_file();
}
