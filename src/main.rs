#![windows_subsystem = "windows"]
mod utils;
mod ui;

use ui::Styling;
use iced::Settings;
use iced::Sandbox;

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (800, 600);
    settings.window.resizable = false;
    Styling::run(settings);
}
