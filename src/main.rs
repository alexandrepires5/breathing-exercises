use std::io;

fn intro_text() {
    println!("Welcome to breathing exercises in rust!");
    println!("This program was made to help you guide through the breathing exercises that we have programmed!");
    println!("You have 2 options:");
    println!("1 - Box breathing");
    println!("2 - Wim Hof breathing");
}

fn main() {
    intro_text();
    println!("Choose your option!: ");
    let mut user_option = String::new();
    io::stdin().read_line(&mut user_option).expect("Failed to read input!");
    println!("Your choice was: {}", user_option);
}