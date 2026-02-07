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

fn approx_pi(n: u64) -> f64 {
    let mut pi_series = 0.;

    // 1 term = 14 digits
    let digits_per_term: i32 = (n as i32) - 14;
    let mut terms: u64 = 0;
    if digits_per_term <= 0 {
        terms = 1
    } else if digits_per_term % 14 == 0{
        terms = (digits_per_term / 14) as u64
    } else {
        terms = ((digits_per_term as f32) / (14.)).ceil() as u64
    }

    for k in 0..terms {
        let alt_one = (-1 as f64).powf(k as f64);
        let numerator = fact_to_float(6 * k) * (545140134. * k as f64 + 13591409.);
        let denomerator = fact_to_float(3 * k) * fact_to_float(k).powf(3.) *
        (640320 as f64).powf((3 * k) as f64 + (3./2.));
        pi_series += (alt_one * numerator) / denomerator;
    }

    let pi = (pi_series * 12.).powf(-1.);
    Decimal::from_f64_retain(pi).unwrap().round_dp(n as u32).to_f64().unwrap()
}



fn main() {
    let n = input();
    let res = approx_pi(n);
    println!("{}", res);
    let pi_assert = Decimal::from_f64_retain(f32::consts::PI as f64).unwrap().round_dp(n as u32).to_f64().unwrap();
    println!("{}", pi_assert);
    let assert_bool =  pi_assert == res;
    println!("{}", assert_bool)
}
