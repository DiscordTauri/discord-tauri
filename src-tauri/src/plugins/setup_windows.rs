/// PLUGIN: SETUP WINDOWS
/// NAME: setupwindows
/// IMPORT AS: SetupWindowsPlugin
/// 
/// AUTHOR: DrPuc
/// DOES: It centers and all Tauri windows and maximizes the main window.

use tauri::{plugin::{Plugin}, Params, Window, Invoke};

// This is needed for the plugin
pub struct SetupWindowsPlugin<P: Params> {
  invoke_handler: Box<dyn Fn(Invoke<P>) + Send + Sync>,
}

impl<P: Params> SetupWindowsPlugin<P> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![]),
    }
  }
}

impl<P: Params> Plugin<P> for SetupWindowsPlugin<P> {
  // The plugin name
  fn name(&self) -> &'static str {
    "setupwindows"
  }

  // The JS script to evaluate on initialization
  fn initialization_script(&self) -> Option<String> {
    Some(String::from("console.log('[discord-tauri] SetupWindows loaded.');"))
  }

  // Callback invoked when a Window is created
  fn created(&mut self, window: Window<P>) {
    // Create a new thread so we don't panic while using the window
    std::thread::spawn(move || {
      // Get the label of the window
      let label = window.label();

      // Center the splashscreen window
      if label.to_string() == "splashscreen" {
        window.center().unwrap();
      }

      // Maximize the main window
      if label.to_string() == "main" {
        window.maximize().unwrap();
      }
    });
  }

  // Extend the invoke handler
  fn extend_api(&mut self, message: Invoke<P>) {
    (self.invoke_handler)(message)
  }
}