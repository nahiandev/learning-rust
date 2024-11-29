fn main() {
    let s1 = String::from("hello"); // s1 is the owner of this String
    let s2 = s1; // s2 takes ownership of the String, s1 is no longer valid

    println!("{}", s2); // This works
    // println!("{}", s1); // This would cause an error because s1 is no longer valid
}
