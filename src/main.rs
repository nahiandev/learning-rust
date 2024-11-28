fn main() {
    for i in (1..=500).step_by(71)
    {
        print!("{}\n", i);
    }
}


// fn is_palindrome(s: &str) -> bool {
//     let s = s.to_lowercase().replace(" ", "");
//     s == s.chars().rev().collect::<String>()
// }
