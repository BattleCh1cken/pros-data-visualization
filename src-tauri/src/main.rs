// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use btleplug::platform::{Adapter, Peripheral};

mod brain;

struct Test(i32);

#[tauri::command]
fn test(state: tauri::State<'_, brain::State>) {
    state.0.lock().unwrap().connected = true;
    println!("{:?}", state);
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .manage(brain::State(Default::default()))
        .invoke_handler(tauri::generate_handler![
            brain::find_brains,
            brain::setup,
            test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
