use cpal::traits::DeviceTrait;
mod devices;
mod stream;

fn main() {
    devices::print_devices();

    let input_devices = devices::input_devices().unwrap();

    println!("\nInput Devices:");
    for device in &input_devices {

        println!("  Input Device: {}", device.name().unwrap_or_else(|_| "Unknown device".to_string()));
    }
    println!("\nThe length of inputs: {}", input_devices.len());

}
