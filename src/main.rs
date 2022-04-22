// Number guessing game
// You select a number
// And the computer needs to guess it
// You tell the computer if the number is correct
// Or if it is too high or too low
// The computer will have 3 guesses
// If the computer guesses correctly, you lose
// If the computer guesses wrong in 3 tries, you win

// Import the random module
// Import the std module
use std::io;
use rand::Rng;

fn main(){
    println!("Welcome to the number guessing game!");
    println!("The rules are simple: You select a number and the computer needs to guess it. \nYou tell the computer if the number is correct or too high or too low. \n The computer will have 3 guesses. \nIf the computer guesses correctly, you lose. \nIf the computer guesses wrong in 3 tries, you win.");
    println!("Let's begin!");
    println!("Please enter a number between 1 and 10.");
    // Get user input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    // Convert user input to an integer
    // Create the computer's guess
    let mut rng = rand::thread_rng();
    let mut computer_guess: i32 = rng.gen_range(0..10);
    // Create a counter for the number of guesses
    let mut guess_counter = 0;
    while guess_counter < 3 {
        println!("Is this your number (respond whit y/n) {}?", computer_guess);
        let mut chcek_if_correct = String::new();
        std::io::stdin().read_line(&mut chcek_if_correct).expect("Failed to read line");
        // If the user says yes, the computer wins
        if chcek_if_correct.trim() == "y" {
            println!("The computer won!");
            break;
        }
        else if chcek_if_correct.trim() == "n" {
            let new_computer_guess: i32 = rng.gen_range(0..10);
            computer_guess = new_computer_guess;
            guess_counter += 1;
            continue;
        }
    }
    println!("The computer lost...\nSo it means you win!!!!!!")

}