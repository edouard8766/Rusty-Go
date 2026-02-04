mod game;
mod board;

use eframe::egui;
use game::Go;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Go Game",
        options,
        Box::new(|_cc| Ok(Box::new(Go::default()))),
    )
}
