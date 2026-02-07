use core::f32;
use std::io;
use rust_decimal::{Decimal, prelude::ToPrimitive};

fn input() -> u64 {
    println!("Please enter a number");
    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Please input an integer");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn factorial(a: u64) -> u64 {
    match a {
        0 => 1,
        _ => a * factorial(a - 1)
    }
}

fn fact_to_float (a: u64) -> f64 {
    factorial(a) as f64
}

fn approx_e(n: u64) -> f64 {
    let mut e_series = 0.;

    for i in 0..(n + 10) {
        e_series += 1. / fact_to_float(i)
    } 
    Decimal::from_f64_retain(e_series).unwrap().round_dp(n as u32).to_f64().unwrap()
}

fn main() {
    let a = approx_e(10);
    println!("{}", a); 
}
