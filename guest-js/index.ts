import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:android_widget_counter|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function getCounter(): Promise<string | null> {  
  return await invoke('plugin:android_widget_counter|get_counter')
}
