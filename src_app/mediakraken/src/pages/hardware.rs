use dioxus::prelude::*;

use crate::api::ApiClient;
use crate::models::HardwareDevice;

#[derive(Clone, Debug, PartialEq)]
enum DevicesState {
    Idle,
    Loading,
    Loaded(Vec<HardwareDevice>),
    Error(String),
}

#[component]
pub fn HardwarePage(client: ApiClient) -> Element {
    let mut state = use_signal(|| DevicesState::Idle);
    let mut last_message = use_signal(String::new);

    let load = {
        let client = client.clone();
        move |_| {
            let client = client.clone();
            spawn(async move {
                state.set(DevicesState::Loading);
                let next = match client.list_hardware().await {
                    Ok(list) => DevicesState::Loaded(list.devices),
                    Err(message) => DevicesState::Error(message),
                };
                state.set(next);
            });
        }
    };

    rsx! {
        section { class: "page",
            h2 { "Hardware control" }
            p { class: "muted",
                "Adjust volume, mute, and power for connected devices."
            }
            button { class: "primary", onclick: load, "Refresh devices" }
            if !last_message().is_empty() {
                p { class: "muted", "{last_message()}" }
            }
            DevicesView {
                state: state(),
                client: client.clone(),
                on_update: move |device: HardwareDevice| {
                    last_message.set(format!("Updated {}", device.name));
                    if let DevicesState::Loaded(mut current) = state() {
                        if let Some(slot) = current.iter_mut().find(|d| d.id == device.id) {
                            *slot = device;
                        }
                        state.set(DevicesState::Loaded(current));
                    }
                },
                on_error: move |message: String| {
                    last_message.set(message);
                },
            }
        }
    }
}

#[component]
fn DevicesView(
    state: DevicesState,
    client: ApiClient,
    on_update: EventHandler<HardwareDevice>,
    on_error: EventHandler<String>,
) -> Element {
    match state {
        DevicesState::Idle => rsx! {
            p { class: "muted", "Tap refresh to list devices." }
        },
        DevicesState::Loading => rsx! {
            p { class: "muted", "Discovering devices..." }
        },
        DevicesState::Error(message) => rsx! {
            p { class: "error", "{message}" }
        },
        DevicesState::Loaded(devices) => {
            if devices.is_empty() {
                rsx! { p { class: "muted", "No devices found." } }
            } else {
                rsx! {
                    div { class: "device-list",
                        for device in devices {
                            DeviceCard {
                                key: "{device.id}",
                                device: device.clone(),
                                client: client.clone(),
                                on_update: on_update,
                                on_error: on_error,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DeviceCard(
    device: HardwareDevice,
    client: ApiClient,
    on_update: EventHandler<HardwareDevice>,
    on_error: EventHandler<String>,
) -> Element {
    let device_id = device.id.clone();
    let toggle_power = {
        let client = client.clone();
        let device_id = device_id.clone();
        let target_powered = !device.powered;
        move |_| {
            let client = client.clone();
            let device_id = device_id.clone();
            spawn(async move {
                match client.set_device_power(&device_id, target_powered).await {
                    Ok(updated) => on_update.call(updated),
                    Err(message) => on_error.call(message),
                }
            });
        }
    };

    let toggle_mute = {
        let client = client.clone();
        let device_id = device_id.clone();
        let target_muted = !device.muted;
        move |_| {
            let client = client.clone();
            let device_id = device_id.clone();
            spawn(async move {
                match client.set_device_mute(&device_id, target_muted).await {
                    Ok(updated) => on_update.call(updated),
                    Err(message) => on_error.call(message),
                }
            });
        }
    };

    let on_volume_input = {
        let client = client.clone();
        let device_id = device_id.clone();
        move |event: FormEvent| {
            let value = event.value();
            let Ok(parsed) = value.parse::<u8>() else {
                return;
            };
            let client = client.clone();
            let device_id = device_id.clone();
            spawn(async move {
                match client.set_device_volume(&device_id, parsed).await {
                    Ok(updated) => on_update.call(updated),
                    Err(message) => on_error.call(message),
                }
            });
        }
    };

    let volume_value = device.volume.unwrap_or(0);
    let volume_label = device
        .volume
        .map(|v| format!("{v}"))
        .unwrap_or_else(|| "—".to_string());
    let power_label = if device.powered { "Power off" } else { "Power on" };
    let mute_label = if device.muted { "Unmute" } else { "Mute" };
    let power_state = if device.powered { "on" } else { "off" };

    rsx! {
        article { class: "device-card",
            header {
                h3 { "{device.name}" }
                span { class: "muted", "{device.kind} · {power_state}" }
            }
            div { class: "device-controls",
                button { onclick: toggle_power, "{power_label}" }
                button { onclick: toggle_mute, "{mute_label}" }
            }
            label { "Volume ({volume_label})"
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "{volume_value}",
                    oninput: on_volume_input,
                }
            }
        }
    }
}
