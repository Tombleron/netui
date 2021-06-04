mod parser;
mod dump;
mod ui;
mod app;

use app::App;
use console_engine::ConsoleEngine;

fn main() {
    let engine = ConsoleEngine::init_fill(100);

    let app = App::new(engine, 5, "wlan0".to_string());

    match app {
        Ok(mut  app) => { app.start().unwrap() },
        Err(_) => {},
    }
}
