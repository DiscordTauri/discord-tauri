#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod plugins;

// Tauri imports
use tauri::Manager;

// Plugin imports
use plugins::{setup_windows::SetupWindowsPlugin, add_window_bar::WindowBarPlugin};

/// This command:
/// - Shows the splashscreen window, because the user could see a blank window for a second otherwise
/// - Is accessible with `show_splashscreen`
#[tauri::command]
fn show_splashscreen(window: tauri::Window) {
  // Show the splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.show().unwrap();
  }
}

/// This command:
/// - Closes the splashscreen window
/// - Un-hides the main window
/// - Is accessible with `close_splashscreen`
#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
  // Close the splashscreen
  window.get_window("splashscreen").unwrap().close().unwrap();
  // Show the main window
  window.get_window("main").unwrap().show().unwrap();
}

fn main() {
  // Instance plugins
  let setup_windows_plugin = SetupWindowsPlugin::new();
  let window_bar_plugin = WindowBarPlugin::new();

  // Start Tauri
  tauri::Builder::default()
    // Register the command
    .invoke_handler(tauri::generate_handler![show_splashscreen, close_splashscreen])
    // Register the plugins
    .plugin(setup_windows_plugin)
    .plugin(window_bar_plugin)
    .run(tauri::generate_context!())
    .expect("Error running discord-tauri");
}