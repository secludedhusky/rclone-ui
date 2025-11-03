// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
  Manager, WebviewUrl, WebviewWindowBuilder,
};

fn create_or_show_main(app: &tauri::AppHandle) {
  if let Some(win) = app.get_webview_window("main") {
    let _ = win.show();
    let _ = win.set_focus();
    return;
  }

  // If your frontend builds `tray.html`, change to "tray.html" below.
  let _ = WebviewWindowBuilder::new(
      app,
      "main",
      WebviewUrl::App("index.html".into())
    )
    .title("Rclone UI")
    .width(1024.0)
    .height(700.0)
    .resizable(true)
    .visible(true) // show when first created
    .center()
    .build();
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Build a simple tray menu
      let open = MenuItemBuilder::with_id("open", "Open").build(app)?;
      let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
      let menu = MenuBuilder::new(app)
        .items(&[&open, &quit])
        .build()?;

      // Create the tray icon (this keeps the app alive even with no windows)
      let _tray: TrayIcon = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(move |app, event| {
          match event.id().as_ref() {
            "open" => create_or_show_main(app),
            "quit" => app.exit(0),
            _ => {}
          }
        })
        .on_tray_icon_event(|app, e| {
          // Left click also opens the window
          if let TrayIconEvent::Click { .. } | TrayIconEvent::DoubleClick { .. } = e {
            create_or_show_main(app);
          }
        })
        .build(app)?;

      // Optional: create window immediately on startup (helpful during debugging)
      // create_or_show_main(app);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running Rclone UI");
}
