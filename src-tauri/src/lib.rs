mod router;
use tauri::{Manager, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_axum::init(router::router()))
        .setup(|app| {
            // Get the main window
            let window = app.get_webview_window("main").unwrap();
            // Navigate to the Axum server
            window.navigate("axum://localhost/".parse().unwrap())?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
