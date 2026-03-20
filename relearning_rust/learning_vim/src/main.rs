fn fizz_buzz() {
    for i in 0..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        }
    }
}

fn reverse_string(n : String) -> String {
    let mut buf = String::new();

    for i in n.chars().rev() {
        buf.push(i);
    }
    buf
}

fn count_vowels(n : String) -> usize {
    let mut s = 0;
    let vowels = vec!["a", "e", "i", "o", "u"].iter().map(|x| x.to_string()).collect::<Vec<String>>();
    for i in n.chars() {
        if vowels.contains(&i.to_string()) {
            s += 1;
        }
    }
    s
}

fn main() {
    println!("Hello, world!");
    fizz_buzz();
    let s = String::from("Sasaki");
    println!("{}", reverse_string(s.clone()));
    println!("{}", count_vowels(s.clone()));
    println!("Wow this is awesome");
}
