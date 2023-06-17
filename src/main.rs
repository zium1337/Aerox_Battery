use std::time::Duration;
use std::thread;
use aerox_5::{Device, DeviceError};
mod battery_tray;
use crate::battery_tray::{TrayHandler, BatteryTray};

fn pair_device() -> Device {
    loop {
        match Device::new() {
            Ok(device) => break device,
            Err(error) => {
                eprintln!("{error}");
            }
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn handle_error(error: DeviceError, device: &mut Device, tray_handler: &mut TrayHandler) {
    match error {
        DeviceError::HidError(hidapi::HidError::HidApiError { message }) => {
            if message == "No such device" {
                eprintln!("No device found.");
                tray_handler.set_status("No device found.");
                *device = pair_device();
            } else {
                eprintln!("{message}");
            }
        }
        DeviceError::NoDeviceFound() => {
            eprintln!("{}", DeviceError::NoDeviceFound());
            tray_handler.set_status( &DeviceError::NoDeviceFound().to_string());
        }
        DeviceError::MouseOff() => {
            eprintln!("{}", DeviceError::MouseOff());
            tray_handler.set_status(&DeviceError::MouseOff().to_string());
        }
        error => {
            eprintln!("{error}");
        }
    }
}

fn main() {
    let mut tray_handler = TrayHandler::new(BatteryTray::new());
    let mut device = pair_device();
    tray_handler.update(&device);

    // Run loop
    loop {
        match device.update_battery_state() {
            Ok(_) => {
                tray_handler.clear_status();
                tray_handler.update(&device);
            },
            Err(error) => {
                handle_error(error, &mut device, &mut tray_handler);
                thread::sleep(Duration::from_secs(10));
                continue;
            },
        };
        thread::sleep(Duration::from_secs(10));
    }
}
