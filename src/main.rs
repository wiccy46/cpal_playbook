use cpal::traits::DeviceTrait;
mod devices;
mod stream;
mod read_wav;

use read_wav::read_wave_file;

fn main() {
    devices::print_devices();

    let input_devices = devices::input_devices().unwrap();

    println!("\nInput Devices:");
    for device in &input_devices {

        println!("  Input Device: {}", device.name().unwrap_or_else(|_| "Unknown device".to_string()));
    }
    println!("\nThe length of inputs: {}", input_devices.len());

    let (samples, sample_rate) = read_wave_file("./examples/speech_with_artificial_reverb.wav").unwrap();
    println!("Sample rate: {}", sample_rate);
    println!("Number of samples: {}", samples.len());

}
