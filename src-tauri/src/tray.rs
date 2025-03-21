use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, MouseButton, TrayIconEvent};
use tauri::{App, Manager};

pub fn create_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
  let online_stat = MenuItem::with_id(
    app,
    "online_stat",
    "Online Stat",
    false,
    None::<&str> 
  )?;

  let quit = MenuItem::with_id(
    app, 
    "quit", 
    "Quit", 
    true, 
    Some("CmdOrCtrl+Q"))?;
  
  let menu = Menu::with_items(app, &[
    &online_stat,
    &quit])?;

  TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu).show_menu_on_left_click(false)
    .on_tray_icon_event(|icon, event| match event {
      TrayIconEvent::Click {
        button: MouseButton::Left,
        .. 
      } => {
        let app = icon.app_handle();
        app.webview_windows().get("main").unwrap().show().unwrap();
      },
      _ => {}
    })
    .on_menu_event(|app, event| match event.id.as_ref() {
      "quit" => {
        println!("tray: quit menu item was clicked");
        app.exit(0);
      }
      _ => {
        eprintln!("tray: menu item {:?} not handled", event.id);
      }
    })
    .build(app)?;

  Ok(())
}