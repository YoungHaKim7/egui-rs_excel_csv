#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

struct MyApp {
    grid: Vec<Vec<String>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            grid: vec![vec!["".to_string(); 10]; 10],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("excel_grid").show(ui, |ui| {
                    for i in 0..self.grid.len() {
                        for j in 0..self.grid[i].len() {
                            ui.text_edit_singleline(&mut self.grid[i][j]);
                        }
                        ui.end_row();
                    }
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Egui Excel",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
