/// PLUGIN: ADD WINDOW BAR
/// NAME: windowbar
/// IMPORT AS: WindowBarPlugin
///
/// AUTHOR: DrPuc
/// DOES: It injects the Discord window bar into the main window.
use tauri::{plugin::Plugin, Invoke, PageLoadPayload, Runtime, Window};

use std::thread::sleep;
use std::time::Duration;

pub struct WindowBarPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> WindowBarPlugin<R> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![]),
    }
  }
}

impl<R: Runtime> Plugin<R> for WindowBarPlugin<R> {
  // The plugin name
  fn name(&self) -> &'static str {
    "windowbar"
  }

  // The JS script to evaluate on initialization
  fn initialization_script(&self) -> Option<String> {
    Some(String::from(
      "console.log('[DISCORD-TAURI v0.3] WindowBar loaded.');",
    ))
  }

  // Callback invoked when the webview performs a navigation
  fn on_page_load(&mut self, window: Window<R>, _: PageLoadPayload) {
    // If the page load isn't from the main window, return
    if window.label().to_string() != "main" {
      return;
    }

    // Make a new thread so we can wait on it
    std::thread::spawn(move || {
      // Wait so we can ensure the page is loaded
      sleep(Duration::from_millis(100));

      // Download the window bar and save in window_bar_html
      let window_bar_html = reqwest::blocking::get("https://raw.githubusercontent.com/DiscordTauri/discord-tauri/dev/src-tauri/src/plugins/window_bar.html")
        .expect("Error downloading the Window Bar")
        .text()
        .expect("Error downloading the Window Bar");

      // Make the code to inject the window bar
      // Waits until the HTML loads, then injects the window bar before the first HTML element
      let js_code = format_args!(r#"
        // If the HTML is loaded, inject the window bar. If not, wait until it exists and then inject.
        let checkHtml = setInterval(() => {{
          // https://github.com/GooseMod/GooseMod/blob/b0acebfd08359364dbee782c6b12cfde1b57a530/src/index.js#L103
          if (document.querySelectorAll('.flex-1xMQg5.flex-1O1GKY.horizontal-1ae9ci.horizontal-2EEEnY.flex-1O1GKY.directionRow-3v3tfG.justifyStart-2NDFzi.alignStretch-DpGPf3.noWrap-3jynv6 > [type="button"]:last-child').length !== 0) {{
            // Inject the window bar before the first HTML element
            document.querySelector('#app-mount > div.app-1q1i1E').insertAdjacentHTML('beforebegin', `{}`);
            
            // Now, we have a visual window bar but it doesn't work. So we inject event listeners to it's buttons.
            // Dragging
            document.getElementById("windowBar").addEventListener("mousedown", window.__TAURI__.window.appWindow.startDragging);
            
            // Close button
            document.getElementById("closeBtn").addEventListener("click", window.__TAURI__.window.appWindow.hide);
            // Maximize button
            document.getElementById("maxBtn").addEventListener("click", window.__TAURI__.window.appWindow.maximize);
            // Minimize button
            document.getElementById("minBtn").addEventListener("click", window.__TAURI__.window.appWindow.minimize);
            clearInterval(checkHtml);
          }}
        }}, 75);
      "#, window_bar_html)
        .to_string();

      // Execute JS code in the main window
      window.eval(&js_code).unwrap();
    });
  }

  // Extend the invoke handler
  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }
}
