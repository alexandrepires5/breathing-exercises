//! A practice implementation of something so that I can learn Rust language in a more
//! practical manner. Just checking the best practices on how to use the language itself
//! and checking this way of documentation. Seems nice. 


use std::io;
mod breathing_exercises;

/// A display text for the user
fn intro_text() {
    println!("Welcome to breathing exercises in rust!");
    println!("This program was made to help you guide through the breathing exercises that we have programmed!");
    println!("You have 2 options:");
    println!("1 - Box breathing");
    println!("2 - Wim Hof breathing");
}

/// Reads the user input and parses it
fn read_user_choice() -> String {
    println!("Choose your option!: ");
    let mut user_option = String::new();
    io::stdin().read_line(&mut user_option).expect("Failed to read input!");
    user_option.trim().to_lowercase()
}

/// Main function. Gets user input, evaluates that its correct and an exercise
fn main() {
    intro_text();
    let mut valid_choice = false;
    while valid_choice != true {
        let user_option = read_user_choice();
        if user_option.eq("1"){
            valid_choice = true;
            println!("Your choice was: BoxBreathing");
            breathing_exercises::box_breathing();
        }
        else if user_option.eq("2"){
            valid_choice = true;
            println!("Your choice was: Wim Hof breathing");
            breathing_exercises::wim_hof_breathing();
        }
        else {
            println!("You inserted an invalid choice! Exiting");
        }
    }
}

