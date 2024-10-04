use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, DevicesError};

pub fn print_devices() {
    let host = cpal::default_host();

    println!("Input Devices:");
    match host.input_devices() {
        Ok(devices) => {
            for device in devices {
                println!("  Input Device: {}", device.name().unwrap_or_else(|_| "Unknown device".to_string()));
            }
        },
        Err(e) => eprintln!("Error getting input devices: {}", e),
    }

    println!("\nOutput Devices:");
    match host.output_devices() {
        Ok(devices) => {
            for device in devices {
                println!("  Output Device: {}", device.name().unwrap_or_else(|_| "Unknown device".to_string()));
            }
        },
        Err(e) => eprintln!("Error getting output devices: {}", e),
    }
}

pub fn input_devices() -> Result<Vec<Device>, DevicesError> {
    let host = cpal::default_host();

    match host.input_devices() {
        Ok(devices) => Ok(devices.collect()),
        Err(e) => Err(e),
    }

}

