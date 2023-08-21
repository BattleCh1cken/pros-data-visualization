// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(async_closure)]

mod brain;

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
            let handle = app.handle();
            tauri::async_runtime::spawn(async {
                brain::find_brain_loop(handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
