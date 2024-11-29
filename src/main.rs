fn main() {
    let s1 = String::from("hello");

    // Immutable borrow
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable borrow
    let mut s2 = String::from("hello");

    change(&mut s2);
    println!("After change: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
