use std::array::from_fn;
use std::fmt::{Display, Formatter, Write};
use rand::random_range;

pub const TURNS: usize = 6;
pub const DIGITS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Green,  // Correct digit, correct position
    Yellow, // Correct digit, wrong position
    Absent, // Digit not in target
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: u8,
    pub color: Color,
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
            target: from_fn(|_| random_range(0..16)),
            attempt_count: 0,
            attempts: from_fn(|_| {
                from_fn(|_| Cell {
                    value: 0,
                    color: Color::Absent,
                })
            }),
        }
    }

    pub fn new_attempt(&mut self, attempt: &[u8; DIGITS]) -> bool {
        if self.out_of_attempts() {
            return false;
        }

        let row = self.attempt_count;

        // count target digits that were not matched exactly (available for yellow)
        let mut remaining = [0u8; 16];
        for i in 0..DIGITS {
            if attempt[i] != self.target[i] {
                remaining[(self.target[i] & 0xF) as usize] += 1;
            }
        }

        let mut correct_guess = true;

        for i in 0..DIGITS {
            let digit = attempt[i] & 0xF;
            let cell = &mut self.attempts[row][i];
            cell.value = digit;

            if digit == self.target[i] {
                cell.color = Color::Green;
            } else {
                correct_guess = false;
                if remaining[digit as usize] > 0 {
                    remaining[digit as usize] -= 1;
                    cell.color = Color::Yellow;
                } else {
                    cell.color = Color::Absent;
                }
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
        let mut target = String::new();

        for i in 0..DIGITS {
            let _ = write!(target, "{:X}", self.target[i]);
        }

        target
    }

    /// The scored cell at (row, col), or None if that row hasn't been guessed yet.
    pub fn cell(&self, row: usize, col: usize) -> Option<(u8, Color)> {
        if row < self.attempt_count && col < DIGITS {
            let c = &self.attempts[row][col];
            Some((c.value, c.color))
        } else {
            None
        }
    }


    /// Returns Ok(true) if the guess was correct.
    pub fn parse_attempt(&mut self, input: &str) -> Result<bool, &'static str> {
        if self.out_of_attempts() {
            return Err("No attempts left");
        }

        let chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

        if chars.len() != DIGITS {
            return Err("Enter exactly 6 hex digits");
        }

        let mut attempt = [0u8; DIGITS];

        for (i, ch) in chars.iter().enumerate() {
            match ch.to_digit(16) {
                Some(digit) => attempt[i] = digit as u8,
                None => return Err("Only 0-9 and A-F are allowed"),
            }
        }

        Ok(self.new_attempt(&attempt))
    }
}

// For printing in the terminal
impl Display for Board {
    fn fmt(&self, board_formatter: &mut Formatter) -> std::fmt::Result {
        for i in 0..self.attempt_count {
            for j in 0..DIGITS {
                let cell = &self.attempts[i][j];
                match cell.color {
                    Color::Green => write!(board_formatter, "\x1b[92m{:X}\x1b[0m ", cell.value)?,
                    Color::Yellow => write!(board_formatter, "\x1b[93m{:X}\x1b[0m ", cell.value)?,
                    Color::Absent => write!(board_formatter, "{:X} ", cell.value)?,
                }
            }
            writeln!(board_formatter)?;
        }
        Ok(())
    }
}