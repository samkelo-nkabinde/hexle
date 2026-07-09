pub mod hexle;
use std::io::{self, Write};

use crate::hexle::Board;

fn main() {
    let mut hexle = Board::new();
    let mut input: String = String::new();

    println!("Try your luck!");

    loop {
        if hexle.out_of_attempts() {
            break; 
        }

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Failed to read user input");
        
        if let Err(err) = hexle.parse_attempt(&input) {
            println!("Error: {}", err);
            input.clear();
            continue;
        }

        print!("{}", hexle);
        io::stdout().flush().unwrap();

        input.clear();
    }
}