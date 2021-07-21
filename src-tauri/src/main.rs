#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod plugins;

// Tauri imports
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

// Plugin imports
use plugins::{add_window_bar::WindowBarPlugin, splashscreen::SplashscreenPlugin};

fn main() {
  // Instance plugins
  let splashscreen_plugin = SplashscreenPlugin::new();
  let window_bar_plugin = WindowBarPlugin::new();

  // We can't make a separate file for a Desktop Tray yet, so we make it here
  // Create an item of the context menu called `quit` with the string `Quit Discord`
  let quit_item = CustomMenuItem::new("quit".to_string(), "Quit Discord");
  // Create a new context menu with the item above
  let context_menu = SystemTrayMenu::new().add_item(quit_item);
  // Create a new system tray with the context menu above
  let desktop_tray = SystemTray::new().with_menu(context_menu);

  // Start Tauri
  tauri::Builder::default()
    // Register the commands
    .invoke_handler(tauri::generate_handler![])
    // Register the plugins
    .plugin(splashscreen_plugin)
    .plugin(window_bar_plugin)
    // Register the desktop tray
    .system_tray(desktop_tray)
    // Add events for the desktop tray
    .on_system_tray_event(|app, event| match event {
      // If the event is a left click into the icon,
      // verify that the window is closed and re-open it
      // while setting it up again (Tauri doesn't save
      // the status of hidden windows).
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        let window = app.get_window("main").unwrap();
        // If the window is closed
        if !window.is_visible().unwrap() {
          // Re-open the window
          window.show().unwrap();
          // Maximize it
          window.maximize().unwrap();
        }
        // Even if the window isn't closed, it could be minimized in the taskbar; set it as focused
        else {
          window.set_focus().unwrap();
        }
      }
      // If the event is a click to an item
      SystemTrayEvent::MenuItemClick { id, .. } => {
        // We compare the name of the item to...
        match id.as_str() {
          // `quit` is the item with the string `Quit Discord`
          // If the user clicks quit, close discord-tauri
          "quit" => {
            // Exit the process with an error code 0
            std::process::exit(0);
          }
          _ => {}
        }
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("Error running discord-tauri");
}
