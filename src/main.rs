pub mod hexle;
use hexle::*;
use eframe::*;
use catppuccin_egui as cp;

struct Hexle {
    theme: cp::Theme,
    board: Board,
    error_message: String,
    about_window: bool
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hexle",
        options,
        Box::new(|_| Ok(Box::new(Hexle::new() )))
    )
}

impl Hexle {
    fn new() -> Self {
        Self {
            theme: cp::MOCHA,
            board: Board::new(),
            error_message: String::new(),
            about_window: false
        }
    }
}

impl eframe::App for Hexle {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        cp::set_theme(context, self.theme);
        self.about(&context);
        
        egui::CentralPanel::default().show(
            context, |ui| {
                egui::ScrollArea::vertical().show(
                    ui, |ui| {
                    self.header(ui);
                    ui.separator();
                    ui.add_space(10.0);
                });
         });
    }
}

impl Hexle {
    fn header(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            
            // main game title
            let title_text = egui::RichText::new("HEXLE")
                .heading()
                .size(30.0);
            
            // set the title to be clickable
            let title_lablel = egui::Label::new(title_text)
                .sense(egui::Sense::click());
            
            // get when the title is clicked
            let response = ui.add(title_lablel);

            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            // show about  window
            if response.clicked() {
                self.about_window = true;
            }

            ui.add_space(10.0);

            // theme widget
            self.select_theme(ui);
             });
    }
    
    fn select_theme(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
           ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),
               |ui| {
                   egui::ComboBox::from_id_salt("THEME_OPTIONS") // ID for theme widgit
                   .selected_text("Theme")
                   .show_ui(ui, |ui| {
                       ui.selectable_value(&mut self.theme, cp::LATTE, "Latte (Light)");
                       ui.selectable_value(&mut self.theme, cp::FRAPPE, "Frappé");
                       ui.selectable_value(&mut self.theme, cp::MOCHA, "Mocha");
                       ui.selectable_value(&mut self.theme, cp::MACCHIATO, "Macchiato");
                });
            }); 
        });
    }
    
    fn about(&mut self, context: &egui::Context) {
        egui::Window::new("About")
        .open(&mut self.about_window) // pop window up if about is true
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
                ui.hyperlink_to("James", "https://jamesl.me/hexle/index.html")
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
