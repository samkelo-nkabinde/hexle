pub mod hexle;
use hexle::*;
use eframe::*;

struct Hexle {
    theme: catppuccin_egui::Theme,
    board: Board,
    error_message: String 
    about_window: bool
}

impl Hexle {
    fn new() -> Self {
        Self {
            theme: catppuccin_egui::MOCHA,
            board: Board::new(),
            error_message: String::new(),
            about_window: false
        }
    }
}

impl eframe::App for Hexle {
    fn update(&mut self, cxt: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(cxt, self.theme);
        
    }
}

impl Helxe {
    fn about(&mut self) {
        egui::Window::new("About")
        .open(&mut self.about_window)
        .collapsible(false)
        .resizable(true)
        .default_size(egui::vec2(400.0, 300.0 )) // 4:3 aspect ratio
        .show(context, |ui| {
            ui.label(egui::RichText::new("Hexle")
                .heading()
                .size(18.0)
                .strong());
            
            ui.add_space(15.00);
            ui.label("A wordle like game were you have to guess 6 digit  base-16 number");
            ui.label("You only have 6 tries to get the number.");
            ui.label("Green means the digit is correct");
            ui.label("Yellow means the digit is correct but in the wrong position");
            ui.add_space(20.0);
            
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 150.0)/2.0);
                ui.label("Inspired by:");
                ui.hyperlink_to("Jordan", "https://hexle.vercel.app/");
                ui.label("and");
                ui.hyperink_to("James", "https://jamesl.me/hexle/index.html")
            });

            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 150.0)/2.0);
                ui.label("Theme powered by:");
                ui.hyperlink_to("Catppuccin", "https://github.com/catppuccin/egui");
            });
        });
    }
}





// use std::io::{self, Write};


// fn main() {
//     let mut hexle = Board::new();
//     let mut input: String = String::new();

//     println!("Try your luck!");

//     loop {
//         if hexle.out_of_attempts() {
//             break; 
//         }

//         print!("Enter your guess: ");
//         io::stdout().flush().unwrap();

//         io::stdin().read_line(&mut input).expect("Failed to read user input");
        
//         if let Err(err) = hexle.parse_attempt(&input) {
//             println!("Error: {}", err);
//             input.clear();
//             continue;
//         }

//         print!("{}", hexle);
//         io::stdout().flush().unwrap();

//         input.clear();
//     }
// }
