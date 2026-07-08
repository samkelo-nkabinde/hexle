use std::io::{self, Write};
use std::fmt::{Display, Formatter, Result};
use rand::random_range;
use std::array::from_fn;

enum Color { 
    GREEN,  // Correct Digit, correct position
    YELLOW, // Correct Digit, incorrect position
    NONE    // Digit not in target
}

struct Cell {
    value: u8,
    color: Color
}

type Row = [Cell; 6];

struct Board {
    target: [u8; 6],
    attempts: [Row; 6],
    attempt_count: usize,
    max_attempts: usize
}

impl Board {
    fn new(max_attempts: usize) -> Self {
        Board {
            target: from_fn( |_| rand::random_range(0..16)),
            max_attempts: max_attempts,
            attempt_count: 0,
            attempts: from_fn( |_| { 
                from_fn( |_| Cell {
                    value: 0, 
                    color: Color::NONE
                })
            })
        }
    }

    fn new_attempt(&mut self, attempt: &[u8; 6]) {
        for i in 0..6 {

            self.attempts[self.attempt_count][i].value = attempt[i];

            if attempt[i] == self.target[i] 
            {
                self.attempts[self.attempt_count][i].color = Color::GREEN;
            }
            else if element_in(&self.target, attempt[i]) 
            {
                self.attempts[self.attempt_count][i].color = Color::YELLOW;
            }
            else
            {
                self.attempts[self.attempt_count][i].color = Color::NONE;
            }

        }

        self.attempt_count += 1;
    }

    fn out_of_attempts(&self) -> bool {
        self.attempt_count >= self.max_attempts 
    }
}

fn element_in<T, const N: usize>(array: &[T; N], element: T) -> bool 
where
    T: PartialEq,
{
    for i in 0..N {
        if array[i] == element {
            return true;
        }
    }

    false
}


impl Display for Board {
    fn fmt(&self, board_formatter: &mut Formatter) ->  Result {
        for i in 0..self.attempt_count {
            for j in 0..6 {
               let _ = match self.attempts[i][j].color {
                    Color::GREEN => {
                        write!(board_formatter, "\x1b[92m{:x}\x1b[0m ", self.attempts[i][j].value)
                    }
                    Color::YELLOW => {
                        write!(board_formatter, "\x1b[93m{:x}\x1b[0m ", self.attempts[i][j].value)
                    }
                    _ => {
                        write!(board_formatter, "{:x} ", self.attempts[i][j].value)
                    }
                };
            }
            write!(board_formatter, "{}", '\n')?
        }
        Ok(())
    }
}

fn main() {

    let mut board: Board = Board::new(6);
    println!("Try your luck");
    let mut input: String = String::new();

    // loop {
    //     if board.out_of_attempts() {
    //         break;
    //     }

    //     print!("Enter Your guess: ", );
    //     io::stdout().flush().unwrap();

    //     io::stdin().read_line(&mut input).expect("Error cannot read input!");

        
    // }
    println!("{}", board);
}