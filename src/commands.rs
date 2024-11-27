use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::AndroidWidgetCounterExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.android_widget_counter().ping(payload)
}

#[command]
pub(crate) fn get_counter<R: Runtime>(
    app: AppHandle<R>,
) -> Result<CounterResponse> {
    app.android_widget_counter().get_counter()
}
