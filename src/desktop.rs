use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<AndroidWidgetCounter<R>> {
  Ok(AndroidWidgetCounter(app.clone()))
}

/// Access to the android-widget-counter APIs.
pub struct AndroidWidgetCounter<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AndroidWidgetCounter<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  pub fn get_counter(&self) -> crate::Result<CounterResponse> {
    Ok(CounterResponse{
      value: None
    })
  }
}
