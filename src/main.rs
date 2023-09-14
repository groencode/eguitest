#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use env_logger;
use eframe::{egui, Theme};

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

struct MyApp {
    is_running: bool,
    is_fullscreen: bool,
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            is_running: true,
            is_fullscreen: false,
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !self.is_running {
            frame.close();
        }
        frame.set_fullscreen(self.is_fullscreen);
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(2f32);
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
