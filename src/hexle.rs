use std::fmt::{Display, Formatter, Write};
use rand::random_range;
use std::array::from_fn;

const TURNS: usize = 6;
const DIGITS: usize = 6;

#[derive(PartialEq)]
enum Color { 
    GREEN,  // Correct Digit, correct position
    YELLOW, // Correct Digit, incorrect position
    NONE    // Digit not in target
}

struct Cell {
    value: u8,
    color: Color
}

type Row = [Cell; DIGITS];

pub struct Board {
    target: [u8; DIGITS],
    attempts: [Row; TURNS],
    attempt_count: usize,
}

impl Board {
    pub fn new() -> Self {
        Board {
            target: from_fn( |_| random_range(0..16)),
            attempt_count: 0,
            attempts: from_fn( |_| { 
                from_fn( |_| Cell {
                    value: 0, 
                    color: Color::NONE
                })
            })
        }
    }

    pub fn new_attempt(&mut self, attempt: &[u8; 6]) -> bool {
        for i in 0..TURNS {

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

        let mut correct_guess: bool = true;
        
        for i in 0..TURNS {         
           if self.attempts[self.attempt_count][i].color != Color::GREEN {
               correct_guess = false;
               break;
           }
        }
        
        self.attempt_count += 1;
        
        correct_guess
    }

    pub fn out_of_attempts(&self) -> bool {
        self.attempt_count >= TURNS
    }
    
    pub fn get_turn_count(&self) -> usize {
        self.attempt_count
    }
    
    pub fn get_target(&self) -> String {
        let mut target: String = String::new();
        
        for i in 0..DIGITS {
            let _ = write!(target, "{:X}", self.target[i]);
        }

        target       
    }  
    
    pub fn parse_attempt(&mut self, input: &str) -> Result<(), &'static str> {
        let input_tokens: Vec<char> = input.split_whitespace()
                                           .map(|c| c.chars().next().unwrap())
                                           .collect();
        if input_tokens.len() < TURNS {
           return Err("Insifficent input");
        }

        let mut attempt: [u8; TURNS] = from_fn( |_| 0);

        for (i, token) in input_tokens.iter().enumerate() {
            let _ = match token.to_digit(16) {
                Some(digit) => {
                    attempt[i] = digit as u8;
                }
                _ => {
                    return Err("incorrect input format");
                }
            };
        }

        self.new_attempt(&attempt);

        Ok(())
    }
}

// check if an element is countained in an array
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

// For print in the terminal
impl Display for Board {
    fn fmt(&self, board_formatter: &mut Formatter) ->  std::fmt::Result {
        for i in 0..self.attempt_count {
            for j in 0..TURNS {
               let _ = match self.attempts[i][j].color {
                    Color::GREEN => {
                        write!(board_formatter, "\x1b[92m{:X}\x1b[0m ", self.attempts[i][j].value)
                    }
                    Color::YELLOW => {
                        write!(board_formatter, "\x1b[93m{:X}\x1b[0m ", self.attempts[i][j].value)
                    }
                    _ => {
                        write!(board_formatter, "{:X} ", self.attempts[i][j].value)
                    }
                };
            }
            write!(board_formatter, "{}", '\n')?
        }
        Ok(())
    }
}
