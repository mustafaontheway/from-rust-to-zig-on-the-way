use std::io;

fn main() {

    let mut user_info = String::new();

    println!("Give us some details: ");

    io::stdin().read_line(&mut user_info).expect("Unexpected error! Try again later...");

    if user_info.trim().is_empty() {

        println!("Please, don't send empty info...")
      
    } else {
        
        println!("Thank you. You sad: \"{}\"", user_info.trim())
    }
}
