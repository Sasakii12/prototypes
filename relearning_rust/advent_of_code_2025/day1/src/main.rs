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

fn main() {
    println!("{:?}", parse_dial("L11".to_string()));
}
