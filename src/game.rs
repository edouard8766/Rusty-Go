use crate::board::Board;
use eframe::egui;

pub struct Go {
    board: Board,
}
impl Default for Go {
    fn default() -> Self {
        Go {
            board: Board::new(19),
        }
    }
}
impl eframe::App for Go {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Go Game!");
            ui.heading("Board Size: 19x19");
        });
    }
}