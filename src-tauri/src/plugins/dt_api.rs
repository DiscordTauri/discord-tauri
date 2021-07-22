/// PLUGIN: DTAPI
/// NAME: dtapi
/// IMPORT AS: DtApiPlugin
///
/// AUTHOR: DrPuc
/// DOES: Adds methods for modifying Discord code.
use tauri::{plugin::Plugin, Invoke, PageLoadPayload, Runtime, Window};

pub struct DtApiPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> DtApiPlugin<R> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![]),
    }
  }
}

impl<R: Runtime> Plugin<R> for DtApiPlugin<R> {
  // The plugin name
  fn name(&self) -> &'static str {
    "dtapi"
  }

  // The JS script to evaluate on initialization
  fn initialization_script(&self) -> Option<String> {
    Some(String::from(
      "console.log('[DISCORD-TAURI] DtApi loaded.');",
    ))
  }

  // Callback invoked when the webview performs a navigation
  fn on_page_load(&mut self, window: Window<R>, _: PageLoadPayload) {
    // If the page load isn't from the main window, return
    if window.label().to_string() != "main" {
      return;
    }

    // Download the compiled version of DtApi and save in dtapi_js
    let dtapi_js = reqwest::blocking::get(
      "https://raw.githubusercontent.com/DiscordTauri/DtApi/main/out/dtapi.js",
    )
    .expect("Error downloading DtApi")
    .text()
    .expect("Error downloading DtApi");

    // Make the code to inject DtApi
    // Wait until webpackJsonp exists, then inject it
    let js_code = format_args!(
      r#"
      // If webpackJsonp exists, execute DtApi. If not, wait until it exists and then execute DtApi
      let checkWebpack = setInterval(() => {{
        if (window.webpackJsonp !== undefined) {{
          {}
          clearInterval(checkWebpack);
        }}
      }}, 75);
    "#,
      dtapi_js
    )
    .to_string();

    // Execute DtApi in the main window
    window.eval(&js_code).unwrap();
  }

  // Extend the invoke handler
  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }
}
