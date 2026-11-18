pub mod hexle;
use hexle::{Board, Color, DIGITS, TURNS};
use eframe::egui::*;
use catppuccin_egui as cp;

struct Hexle {
    theme: cp::Theme,
    board: Board,
    result_window: bool,
    about_window: bool,
    correct: bool,
    input: String,
    error: Option<String>,
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hexle",
        options,
        Box::new(|_| Ok(Box::new(Hexle::new()))),
    )
}

trait ThemeName {
    fn get_name(&self) -> String;
}

impl ThemeName for cp::Theme {
    fn get_name(&self) -> String {
        String::from(match *self {
            cp::FRAPPE => "Frappé",
            cp::MOCHA => "Mocha",
            cp::LATTE => "Latte",
            cp::MACCHIATO => "Macchiato",
            _ => "",
        })
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
            input: String::new(),
            error: None,
        }
    }

    fn game_over(&self) -> bool {
        self.correct || self.board.out_of_attempts()
    }

    fn reset(&mut self) {
        self.board = Board::new();
        self.correct = false;
        self.result_window = false;
        self.input.clear();
        self.error = None;
    }

    fn submit(&mut self) {
        if self.game_over() {
            return;
        }

        match self.board.parse_attempt(&self.input) {
            Ok(correct) => {
                self.input.clear();
                self.error = None;
                self.correct = correct;

                if self.game_over() {
                    self.result_window = true;
                }
            }
            Err(message) => self.error = Some(message.to_string()),
        }
    }
}

impl eframe::App for Hexle {
    fn update(&mut self, context: &Context, _frame: &mut eframe::Frame) {
        cp::set_theme(context, self.theme);
        self.display_about_window(context);
        self.display_results_window(context);

        CentralPanel::default().show(context, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.header(ui);
                ui.separator();
                ui.add_space(10.0);
                self.draw_board(ui);
                ui.add_space(15.0);
                self.input_area(ui);
            });
        });
    }
}

impl Hexle {
    fn header(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            // main game title
            let title_text = RichText::new("HEXLE").heading().size(30.0);

            // set the title to be clickable
            let title_label = Label::new(title_text).sense(Sense::click());

            // get when the title is clicked
            let response = ui.add(title_label);

            if response.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // show about window
            if response.clicked() {
                self.about_window = true;
            }

            // theme widget
            self.select_theme(ui);
        });
    }

    fn select_theme(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("Turns: {}/{}", self.board.get_turn_count(), TURNS))
                    .size(15.0),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // combination box widget for themes
                ComboBox::from_id_salt("THEME_OPTIONS")
                    .selected_text(self.theme.get_name())
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

    fn draw_board(&self, ui: &mut Ui) {
        let cell_size = 56.0;
        let gap = 6.0;
        let width = DIGITS as f32 * cell_size + (DIGITS - 1) as f32 * gap;
        let height = TURNS as f32 * cell_size + (TURNS - 1) as f32 * gap;

        let t = self.theme;
        let dark_text = Color32::from_rgb(30, 30, 46);
        let active_row = if self.game_over() {
            usize::MAX
        } else {
            self.board.get_turn_count()
        };

        ui.vertical_centered(|ui| {
            let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
            let painter = ui.painter_at(rect);

            for r in 0..TURNS {
                for c in 0..DIGITS {
                    let min = rect.min + vec2(c as f32 * (cell_size + gap), r as f32 * (cell_size + gap));
                    let cell_rect = Rect::from_min_size(min, vec2(cell_size, cell_size));

                    let (fill, label, fg) = if let Some((value, color)) = self.board.cell(r, c) {
                        let (fill, fg) = match color {
                            Color::Green => (t.green, dark_text),
                            Color::Yellow => (t.yellow, dark_text),
                            Color::Absent => (t.surface2, t.text),
                        };
                        (fill, Some(format!("{:X}", value)), fg)
                    } else if r == active_row {
                        // the row currently being typed
                        (t.surface1, self.input.chars().nth(c).map(|ch| ch.to_string()), t.text)
                    } else {
                        (t.surface0, None, t.text)
                    };

                    painter.rect_filled(cell_rect, 8.0, fill);

                    if let Some(text) = label {
                        painter.text(
                            cell_rect.center(),
                            Align2::CENTER_CENTER,
                            text,
                            FontId::monospace(26.0),
                            fg,
                        );
                    }
                }
            }
        });
    }

    fn input_area(&mut self, ui: &mut Ui) {
        let game_over = self.game_over();
        let mut submit = false;

        ui.vertical_centered(|ui| {
            ui.add_enabled_ui(!game_over, |ui| {
                let response = ui.add(
                    TextEdit::singleline(&mut self.input)
                        .char_limit(DIGITS)
                        .hint_text("e.g. 3FA09C")
                        .font(FontId::monospace(20.0))
                        .desired_width(170.0),
                );

                // keep only hex digits, uppercase
                self.input.retain(|c| c.is_ascii_hexdigit());
                self.input = self.input.to_uppercase();

                let enter_pressed =
                    response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));

                ui.add_space(8.0);
                let clicked = ui.button("Guess").clicked();

                if enter_pressed || clicked {
                    submit = true;
                    response.request_focus();
                }
            });

            if let Some(error) = &self.error {
                ui.add_space(8.0);
                ui.colored_label(self.theme.red, error);
            }

            ui.add_space(12.0);
            if ui.button("New game").clicked() {
                self.reset();
            }
        });

        if submit {
            self.submit();
        }
    }

    fn display_results_window(&mut self, context: &Context) {
        let window_size = vec2(300.0, 200.0);
        let center = context.content_rect().center();

        let mut open = self.result_window;
        let mut new_game = false;
        let correct = self.correct;
        let target = self.board.get_target();
        let turns = self.board.get_turn_count();

        Window::new("Results")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_pos(center)
            .pivot(Align2::CENTER_CENTER)
            .default_size(window_size)
            .show(context, |ui| {
                ui.vertical_centered(|ui| {
                    if correct {
                        ui.label(
                            RichText::new(format!("Congratulations! You got it in {} turns.", turns))
                                .size(15.0),
                        );
                    } else {
                        ui.label(RichText::new("You're out of attempts!").size(15.0));
                    }

                    ui.add_space(6.0);
                    ui.label(RichText::new(format!("The answer was: {}", target)).size(15.0));
                    ui.add_space(12.0);

                    if ui.button("Play again").clicked() {
                        new_game = true;
                    }
                });
            });

        self.result_window = open;

        if new_game {
            self.reset();
        }
    }

    fn display_about_window(&mut self, context: &Context) {
        Window::new("About")
            .open(&mut self.about_window) // pop window up if about is true
            .collapsible(false)
            .resizable(true)
            .default_size(vec2(300.0, 400.0)) // 3:4 aspect ratio
            .show(context, |ui| {
                ui.label(RichText::new("Hexle").heading().size(18.0).strong());

                ui.add_space(15.0);
                ui.label("A Wordle-like game where you have to guess a 6 digit base-16 number.");
                ui.label("You only have 6 tries to get the number.");
                ui.label("Green means the digit is correct and in the right position.");
                ui.label("Yellow means the digit is in the number but in the wrong position.");
                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    ui.label("Inspired by:");
                    ui.hyperlink_to("Jordan's version", "https://hexle.vercel.app/");
                    ui.hyperlink_to("James' version", "https://jamesl.me/hexle/index.html");
                    ui.add_space(10.0);
                    ui.label("Theme powered by:");
                    ui.hyperlink_to("Catppuccin", "https://github.com/catppuccin/egui");
                });
            });
    }
}