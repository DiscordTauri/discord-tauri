/// PLUGIN: NOTIFICATIONS
/// NAME: notifications
/// IMPORT AS: NotificationsPlugin
///
/// AUTHOR: DrPuc
/// DOES: Patches the Discord notification code to use Tauri.
use tauri::{plugin::Plugin, Invoke, PageLoadPayload, Runtime, Window};

pub struct NotificationsPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> NotificationsPlugin<R> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![]),
    }
  }
}

impl<R: Runtime> Plugin<R> for NotificationsPlugin<R> {
  // The plugin name
  fn name(&self) -> &'static str {
    "notifications"
  }

  // The JS script to evaluate on initialization
  fn initialization_script(&self) -> Option<String> {
    Some(String::from(
      "console.log('[DISCORD-TAURI] Notifications loaded.');",
    ))
  }

  // Callback invoked when the webview performs a navigation
  fn on_page_load(&mut self, window: Window<R>, _: PageLoadPayload) {
    // If the page load isn't from the main window, return
    if window.label().to_string() != "main" {
      return;
    }

    // Make the patch
    let module_code = r#"
      // Request Tauri permission to use notifications
      window.__TAURI__.notification.requestPermission();
      // Get the Discord notification module
      let notificationModule = window.dtapi.webpackModules.findByProps("showNotification");
  
      // Modify the showNotification function in the module
      // notificationPatch() will remove the patch
      let notificationPatch = window.dtapi.patch(notificationModule, "showNotification", (args) => {
        // TODO: The icon doesn't work
        // Show a new Tauri notification with:
        // - args[1]: Title
        // - args[2]: Body
        window.Notification(args[1], {
          body: args[2]
        });
      });
  
      // Add the patch to the list of patches so we can unpatch it later
      window.dtapi.patches.push(notificationPatch);
    "#;

    // Make the code to patch
    // Wait until window.dtapi exists, then patch
    let js_code = format_args!(
      r#"
      // If DtApi exists, patch. If not, wait until it exists and then patch
      let checkDtApi = setInterval(() => {{
        if (window.dtapi !== undefined) {{
          {}
          clearInterval(checkDtApi);
        }}
      }}, 75);
    "#,
      module_code
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
