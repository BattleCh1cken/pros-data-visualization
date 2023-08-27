// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod brain;
mod loggingv2;

#[cfg(test)]
mod tests;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .manage(brain::BtState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            brain::connect,
            brain::authenticate,
            brain::disconnect
        ])
        .setup(|app| {
            let handle1 = app.handle();
            let handle2 = app.handle();

            tauri::async_runtime::spawn(async move {
                brain::find_brain_loop(handle1).await.unwrap();
            });
            tauri::async_runtime::spawn(async move {
                loggingv2::logging_loop(handle2).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
