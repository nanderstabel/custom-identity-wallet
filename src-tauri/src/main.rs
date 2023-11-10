// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use identity_wallet::command::handle_action_2;
use identity_wallet::state::AppState;
use identity_wallet::{initialize_storage, state::actions::Action};

fn main() {
    #[cfg(desktop)]
    run();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_action])
        .setup(move |app| {
            initialize_storage(app.handle()).ok();
            #[cfg(mobile)]
            {
                app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
            }
            Ok(())
        })
        .manage(AppState::default())
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn handle_action<R: tauri::Runtime>(
    action: Action,
    _app_handle: tauri::AppHandle<R>,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    handle_action_2(action, _app_handle, app_state, window).await
}
