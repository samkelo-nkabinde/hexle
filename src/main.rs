pub mod hexle;
use hexle::*;
use eframe::egui::*;
use catppuccin_egui as cp;

struct Hexle {
    theme: cp::Theme,
    board: Board,
    result_window: bool,
    about_window: bool,
    correct: bool, 
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hexle",
        options,
        Box::new(|_| Ok(Box::new(Hexle::new() )))
    )
}

trait ThemeName {
    fn get_name(&self) -> String;
}

impl ThemeName for cp::Theme {
    fn get_name(&self) -> String {
        String::from(
            match *self {
                cp::FRAPPE => "Frappé",
                cp::MOCHA => "Mocha",
                cp::LATTE => "Latte",
                cp::MACCHIATO => "Macchiato",
                _ => ""
            }
        )
    }
}

impl Hexle {
    fn new() -> Self {
        Self {
            theme: cp::MOCHA,
            board: Board::new(),
            about_window: false,
            result_window: false,
            correct: false,
        }
    }

    // fn open_result_window(&mut self) -> bool {
    //     if self.correct {
    //         self.result = true;
    //     }
    //     else if self.board.out_of_attempts() {
    //         self.result_window = true;
    //     }
    //     else {
    //         self.result_window = false
    //     }
    // }
}

impl eframe::App for Hexle {
    fn update(&mut self, context: &Context, _frame: &mut eframe::Frame) {
        cp::set_theme(context, self.theme);
        self.display_about_window(&context);
        self.display_results_window(&context);
        
        CentralPanel::default().show(
            context, |ui| {
                ScrollArea::vertical().show(
                    ui, |ui| {
                    self.header(ui);
                    ui.separator();
                    ui.add_space(10.0);
                    

                    Grid::new("my_single_row_grid")
    .spacing([10.0, 4.0]) // [x_spacing, y_spacing]
    .show(ui, |ui| {
        // Column 1
            let square_size = (ui.available_height() / 9.0) - 10.0;

            // 9 squares of square size and 3 spaces of 2.0 width
            let adjust = ((9.0 * square_size) + 9.0) / 2.0;

            // This sets the initial position of the grid
            let initial_position = Pos2::new(
                // Using some pretty basic logic we just get the center and subtract by adjust
                ui.available_width() / 2.0 - adjust,
                // Doing the same here but we need to make sure we have the correct starting position
                ui.next_widget_position().y + ui.available_height() / 2.0 - adjust,
            );

            // This is going to be our position that we continually update as we move through the grid
            let mut pos = initial_position;
            
        let rect = Rect::from_two_pos(pos, pos + vec2(square_size, square_size));
        let response = ui.allocate_rect(rect, Sense::click());

        // Check for clicks
        if response.clicked() {
            println!("The user clicked the square!");
        }

        // You can also check for hover states to change colors
        if response.hovered() {
            // Maybe draw a highlight over the square
        }
        // Finish the row
        ui.end_row(); 
    });
                });
         });
    }
}

impl Hexle {
    fn header(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            
            // main game title
            let title_text = RichText::new("HEXLE")
                .heading()
                .size(30.0);
            
            // set the title to be clickable
            let title_lablel = Label::new(title_text)
                .sense(Sense::click());
            
            // get when the title is clicked
            let response = ui.add(title_lablel);

            if response.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // show about  window
            if response.clicked() {
                self.about_window = true;
            }

            // theme widget
            self.select_theme(ui);
             });
    }
    
    fn select_theme(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
        
           ui.label(RichText::new(format!("Turns: {}", self.board.get_turn_count())).size(15.0));
           
           ui.with_layout(Layout::right_to_left(Align::Center),
               |ui| {
                     
                   // combination box widget for themes
                   ComboBox::from_id_salt("THEME_OPTIONS") // ID for theme widgit
                   .selected_text(self.theme.get_name()) // display current theme name
                   .show_ui(ui, |ui| {
                       ui.selectable_value(&mut self.theme, cp::LATTE, "Latte (Light)");
                       ui.selectable_value(&mut self.theme, cp::FRAPPE, "Frappé");
                       ui.selectable_value(&mut self.theme, cp::MOCHA, "Mocha");
                       ui.selectable_value(&mut self.theme, cp::MACCHIATO, "Macchiato");
                    });

                    ui.label(RichText::new("Theme").size(15.0));
            }); 
        });
    }

    fn display_results_window(&mut self, context: &Context) {
        let window_size = vec2(300.0, 400.0);
        let center = context.content_rect().center();
        
        Window::new("Results")
        .open(&mut self.result_window)
        .collapsible(false)
        .resizable(false)
        .default_pos(center)
        .pivot(Align2::CENTER_CENTER)
        .default_size(window_size)
        .show(context, |ui| {
            if self.correct {
                ui.label(RichText::new("Congradulations! you got it").size(15.0));
            }
            else {
                ui.label(RichText::new(format!("You are of attempts!\nThe answer was: {}", self.board.get_target())).size(15.0));
            }
        }); 
    }
    
    fn display_about_window(&mut self, context: &Context) {
        Window::new("About")
        .open(&mut self.about_window) // pop window up if about is true
        .collapsible(false)
        .resizable(true)
        .default_size(vec2(300.0, 400.0 )) // 3:4 aspect ratio
        .show(context, |ui| {
            ui.label(RichText::new("Hexle")
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
                ui.hyperlink_to("Jordan's version", "https://hexle.vercel.app/");
                ui.label("and");
                ui.hyperlink_to("James' version", "https://jamesl.me/hexle/index.html")
            });

            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 150.0)/2.0);
                ui.label("Theme powered by:");
                ui.hyperlink_to("Catppuccin", "https://github.com/catppuccin/egui");
            });
        });
    }
}





