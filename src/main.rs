fn main() {
    println!("{}", is_palindrome("Hello, world!"));
}


fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase().replace(" ", "");
    s == s.chars().rev().collect::<String>()
}
