fn reverse_string(s : String) -> String {
    s.chars().rev().collect()
}

fn count_vowels(s: String) -> usize {
    let vowels = vec!['a','e','i','o', 'u'];
    s.chars().filter(|c| vowels.contains(c)).count()

}

fn rev_str(str: &str) -> String {
    str.chars().rev().collect()
}

fn is_palindrome(str: &str) -> bool {
    str.to_lowercase() == rev_str(&str).as_str().to_lowercase()
}

fn main() {
    println!("{}", count_vowels(String::from("Mraow")));
    println!("{}",reverse_string(String::from("mraow")));
    println!("{}", is_palindrome("nisioisin"))
}
