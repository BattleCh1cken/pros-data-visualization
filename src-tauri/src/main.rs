// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;
use std::path::PathBuf;
use tauri::Manager;
use tauri::Wry;
use tauri_plugin_store::with_store;
use tauri_plugin_store::StoreBuilder;

mod brain;
mod logging;

#[cfg(test)]
mod tests;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .manage(brain::BtState(Default::default()))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            brain::connect,
            brain::authenticate,
            brain::disconnect
        ])
        .setup(|app| {
            let handle1 = app.handle();
            let handle2 = app.handle();

            let mut store = StoreBuilder::new(app.handle(), ".settings.dat".parse()?).build();
            store.insert("a".to_string(), json!("b")).unwrap();
            let bob = store.get("a").unwrap();
            println!("{:?}", bob);

            tauri::async_runtime::spawn(async move {
                brain::find_brain_loop(handle1).await.unwrap();
            });
            tauri::async_runtime::spawn(async move {
                logging::logging_loop(handle2).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
