use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AndroidWidgetCounter;
#[cfg(mobile)]
use mobile::AndroidWidgetCounter;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the android-widget-counter APIs.
pub trait AndroidWidgetCounterExt<R: Runtime> {
  fn android_widget_counter(&self) -> &AndroidWidgetCounter<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidWidgetCounterExt<R> for T {
  fn android_widget_counter(&self) -> &AndroidWidgetCounter<R> {
    self.state::<AndroidWidgetCounter<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("android-widget-counter")
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::get_counter
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let android_widget_counter = mobile::init(app, api)?;
      #[cfg(desktop)]
      let android_widget_counter = desktop::init(app, api)?;
      app.manage(android_widget_counter);
      Ok(())
    })
    .build()
}
