fn main() {
    for i in (-2..=50000).step_by(71)
    {
        if i % 69 == 0 {
            print!("{}\n", i);
        }    
    }
}


// fn is_palindrome(s: &str) -> bool {
//     let s = s.to_lowercase().replace(" ", "");
//     s == s.chars().rev().collect::<String>()
// }
