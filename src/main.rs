mod app;

use crate::app::App;
use std::env;

// use reports::report::*;
use csv::{Reader, Writer};
use std::path::PathBuf;


fn main() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.pop(); // Move up one directory
    current_dir.push("/app/src/inventory.csv");

    let mut app = App::new(current_dir.to_str().unwrap()).unwrap();
    app.run();
}