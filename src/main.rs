#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use env_logger;
use eframe::{egui, Theme};
use egui::{Id, Visuals};
use egui::panel::{Side, TopBottomSide};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        min_window_size: Some(egui::vec2(320.0, 240.0)),
        initial_window_pos: Some(egui::pos2(10.0, 100.0)),
        default_theme: Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}


#[derive(PartialEq)]
enum LeftMenuEnum { First, Second, Third }


struct MyApp {
    is_running: bool,
    is_fullscreen: bool,
    is_popup_shown: bool,
    is_dark: bool,
    left_menu: LeftMenuEnum,
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            is_running: true,
            is_fullscreen: false,
            is_popup_shown: true,
            is_dark: false,
            left_menu: LeftMenuEnum::First,
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl MyApp {

    fn handle_close(&self, frame: &mut eframe::Frame) {
        if !self.is_running {
            frame.close();
        }
    }

    fn handle_popup_window(&mut self, ctx: &egui::Context) {
        if self.is_popup_shown {
            egui::Window::new("Pop window").show(ctx, |ui| {
                ui.label("Started a window!");
                if ui.button("Close").clicked() {
                    self.is_popup_shown = false;
                }
            });
        }
    }

    fn add_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::new(TopBottomSide::Top, Id::new("top_panel"))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Set age to 45").clicked() {
                            self.age = 45;
                        }
                        if ui.button("Close the menu").clicked() {
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Copy").clicked() {}
                        if ui.button("Paste").clicked() {}
                        if self.is_dark {
                            if ui.button("Set light").clicked() {
                                self.is_dark = false;
                            }
                        } else {
                            if ui.button("Set dark").clicked() {
                                self.is_dark = true;
                            }
                        }
                    });
                });
            });
    }

    fn handle_center_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(2f32);

            // Looking for a better way:
            egui::menu::bar(ui, |contents| {
                if contents.button("File").clicked() {
                    self.age = 5;
                }
            });

            // Start of main panel
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            if ui.button("Click add 2 years").clicked() {
                self.age += 2;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            if ui.button("Exit").clicked() {
                self.is_running = false;
            }
            if ui.button("Fullscreen").clicked() {
                self.is_fullscreen = !self.is_fullscreen;
            }

            // A `scope` creates a temporary [`Ui`] in which you can change settings:
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                ui.style_mut().wrap = Some(false);

                ui.label("This text will be red, monospace, and won't wrap to a new line");
                ui.label(egui::RichText::new("Large text").font(egui::FontId::proportional(40.0)));
                ui.add_sized([80.0, 40.0], egui::DragValue::new(&mut self.age));
            }); // the temporary settings are reverted here

        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Create a few panels and controls to show how to create a UI with egui.
        ctx.set_visuals(if self.is_dark { Visuals::dark() } else { Visuals::light() });
        self.handle_close(frame);
        frame.set_fullscreen(self.is_fullscreen);
        self.add_top_panel(ctx);

        egui::SidePanel::new(Side::Left, Id::new("left_panel"))
            .show(ctx, |ui| {
                ui.selectable_value(&mut self.left_menu, LeftMenuEnum::First, "First");
                ui.selectable_value(&mut self.left_menu, LeftMenuEnum::Second, "Second");
                ui.selectable_value(&mut self.left_menu, LeftMenuEnum::Third, "Third");
            });

        match self.left_menu {
            LeftMenuEnum::First => {
                self.handle_center_panel(ctx);
                self.handle_popup_window(ctx);
            },
            LeftMenuEnum::Second => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Second panel");
                });
            },
            LeftMenuEnum::Third => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Third panel");
                });
            }
        }
    }
}
