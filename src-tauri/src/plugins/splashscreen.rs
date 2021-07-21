/// PLUGIN: SPLASHSCREEN
/// NAME: splashscreen
/// IMPORT AS: SplashscreenPlugin
///
/// AUTHOR: DrPuc
/// DOES: Waits for Discord's HTML to hide the splashscreen and show the main window.
use tauri::{plugin::Plugin, Invoke, Runtime, Window};

use std::thread::sleep;
use std::time::Duration;

pub struct SplashscreenPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> SplashscreenPlugin<R> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![]),
    }
  }
}

impl<R: Runtime> Plugin<R> for SplashscreenPlugin<R> {
  // The plugin name
  fn name(&self) -> &'static str {
    "splashscreen"
  }

  // The JS script to evaluate on initialization
  fn initialization_script(&self) -> Option<String> {
    Some(String::from(
      "console.log('[discord-tauri] Splashscreen loaded.');",
    ))
  }

  // Callback invoked when a Window is created
  fn created(&mut self, window: Window<R>) {
    // Create a new thread so we can wait on it
    std::thread::spawn(move || {
      // Wait 500ms
      sleep(Duration::from_millis(500));

      // Close the splashscreen window
      if window.label().to_string() == "splashscreen" {
        window.close().unwrap();
      }

      // Show and maximize the main window
      if window.label().to_string() == "main" {
        window.show().unwrap();
        window.maximize().unwrap();
      }
    });
  }

  // Extend the invoke handler
  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }
}
