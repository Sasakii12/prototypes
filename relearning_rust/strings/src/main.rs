fn fizz_buzz() {
    for i in 0..101 {
        if i % 3 == 0 {
            println!("Fizz");
        }
        else if i % 5 == 0{
            println!("Buzz");
        }
    }
}

fn reverse_string(s : String) -> String {
    s.chars().rev().collect()
}

fn count_vowels(s: String) -> usize {
    let vowels = vec!['a','e','i','o', 'u'];
    s.chars().filter(|c| vowels.contains(c)).count()

}

fn main() {
    println!("{}", count_vowels(String::from("Mraow")));
    println!("{}",reverse_string(String::from("mraow")))
}
