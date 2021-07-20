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
    // Create a new thread so we don't panic while using the window
    std::thread::spawn(move || {
      // If the window isn't the main one, return
      if window.label().to_string() != "main" {
        return;
      }

      // Wait 500ms
      sleep(Duration::from_millis(500));

      // Execute JS code in the main window
      window
        .eval(
          r#"
        // Once the HTML is ready, we call `close_splashscreen`
        window.__TAURI__.invoke('close_splashscreen');
      "#,
        )
        .unwrap();
    });
  }

  // Extend the invoke handler
  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }
}
