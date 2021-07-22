/// PLUGIN: SPLASHSCREEN
/// NAME: splashscreen
/// IMPORT AS: SplashscreenPlugin
///
/// AUTHOR: DrPuc
/// DOES: Waits for the main window to load. Then hides the splashscreen and shows the main window.
use tauri::{plugin::Plugin, Invoke, Manager, PageLoadPayload, Runtime, Window};

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
      "console.log('[DISCORD-TAURI] Splashscreen loaded.');",
    ))
  }

  // Callback invoked when the webview performs a navigation
  fn on_page_load(&mut self, window: Window<R>, _: PageLoadPayload) {
    // If the page load isn't from the main window, return
    if window.label().to_string() != "main" {
      return;
    }

    // Close the splashscreen window
    window.get_window("splashscreen").unwrap().close().unwrap();

    // Show and maximize the main window
    window.get_window("main").unwrap().show().unwrap();
    window.get_window("main").unwrap().set_focus().unwrap();
    window.get_window("main").unwrap().maximize().unwrap();
  }

  // Extend the invoke handler
  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }
}
