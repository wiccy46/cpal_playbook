use cpal::{Device, Stream, BuildStreamError};
use cpal::traits::DeviceTrait;

#[allow(dead_code)]
pub fn make_input_stream(device: &Device) -> Result<Stream, BuildStreamError> {
    let config = device.default_input_config().unwrap();
    let stream = device.build_input_stream(
        &config.config(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            println!("{:?}", &data[0..5]);
        },
        move |err| {
            eprintln!("Error: {}", err);
        },
        None,
        
    )?;

    Ok(stream)
}
