use app::App;
use ratatui::init;
use std::{env::current_dir, io};

mod app;
mod event;
mod filesystem;
mod tasks;
mod ui;

use log::LevelFilter;
use simple_logging::log_to_file;

fn main() -> io::Result<()> {
    let _ = log_to_file("test.log", LevelFilter::Debug);
    let mut terminal = init();
    let app_result = App::new(current_dir().unwrap().display().to_string()).run(&mut terminal);
    ratatui::restore();
    app_result
}
