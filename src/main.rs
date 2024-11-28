// fn main() {
//     for i in (-2..=50000).step_by(71)
//     {
//         if i % 69 == 0 {
//             print!("{}\n", i);
//         }    
//     }
// }



#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello Rust!"
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


// fn is_palindrome(s: &str) -> bool {
//     let s = s.to_lowercase().replace(" ", "");
//     s == s.chars().rev().collect::<String>()
// }
