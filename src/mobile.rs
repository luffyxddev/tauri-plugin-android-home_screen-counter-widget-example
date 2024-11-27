use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_android_widget_counter);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<AndroidWidgetCounter<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.android_widget_counter", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_android_widget_counter)?;
  Ok(AndroidWidgetCounter(handle))
}

/// Access to the android-widget-counter APIs.
pub struct AndroidWidgetCounter<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AndroidWidgetCounter<R> {

  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }

  pub fn get_counter(&self) -> crate::Result<CounterResponse> {
    self
      .0
      .run_mobile_plugin("getCounter", CounterRequest{})
      .map_err(Into::into)
  }

}
