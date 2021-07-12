#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod plugins;

// Tauri imports
use tauri::{Manager, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTray};

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
    .invoke_handler(tauri::generate_handler![show_splashscreen, close_splashscreen])
    // Register the plugins
    .plugin(setup_windows_plugin)
    .plugin(window_bar_plugin)
    // Register the desktop tray
    .system_tray(desktop_tray)
    // Add events for the desktop tray
    .on_system_tray_event(|app, event| match event {
      // If the event is a left click into the icon,
      // verify that the window is closed and re-open it
      // while setting it up again (Tauri doesn't save
      // the status of hidden windows).
      SystemTrayEvent::LeftClick {position: _,size: _,..} => {
        let window = app.get_window("main").unwrap();
        // Create a new thread so we don't panic while using the window
        std::thread::spawn(move || {
          // If the window is hidden, continue
          if !window.is_visible().unwrap() {
            // Set the window up again
            window.center().unwrap();
            window.maximize().unwrap();
            // Re-open the window
            window.show().unwrap();
            // Set the window as focused, otherwise it would stay in the taskbar
            window.set_focus().unwrap();
          }
          // Even if the user didn't close the window, it could be minimized; set it as focused
          window.unminimize().unwrap();
          window.set_focus().unwrap();
        });
      }
      // If the event is a click to an item
      SystemTrayEvent::MenuItemClick {id,..} => {
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