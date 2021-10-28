use std::io;

fn intro_text() {
    println!("Welcome to breathing exercises in rust!");
    println!("This program was made to help you guide through the breathing exercises that we have programmed!");
    println!("You have 2 options:");
    println!("1 - Box breathing");
    println!("2 - Wim Hof breathing");
}

fn read_user_choice() -> String {
    println!("Choose your option!: ");
    let mut user_option = String::new();
    io::stdin().read_line(&mut user_option).expect("Failed to read input!");
    return user_option.trim().to_lowercase();
}

fn main() {
    intro_text();
    let mut valid_choice = false;
    while valid_choice != true{
        let user_option = read_user_choice();
        if !user_option.eq("1") && !user_option.eq("2") {
            println!("You inserted an invalid choice! Exiting");
        }
        else {
            println!("Your choice was: {}", user_option);
            valid_choice = true;
        }
    }
    
    
}