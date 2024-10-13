use hound;
use hound::SampleFormat;

pub fn read_wave_file(filepath: &str) -> Result<(Vec<f32>, u32), Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(filepath)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    println!("sample rate: {}", sample_rate);
    let sample_format = spec.sample_format;
    println!("sample format: {:?}", sample_format);
    let bits_per_sample = spec.bits_per_sample;
    println!("bits per sample: {}", bits_per_sample);

    // Normalize to f32, downsample 24bit to 16bit
    let samples = match (sample_format, bits_per_sample) {
        (SampleFormat::Int, 16) => reader.samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32) // Normalize i16 to f32
            .collect(),
        (SampleFormat::Int, 24) => reader.samples::<i32>() // Read as i32 for 24-bit audio
            .map(|s| (s.unwrap() >> 8) as f32 / i16::MAX as f32) // Normalize 24-bit (shift by 8)
            .collect(),
        (SampleFormat::Int, 32) => reader.samples::<i32>()
            .map(|s| s.unwrap() as f32 / i32::MAX as f32) // Normalize i32 to f32
            .collect(),
        (SampleFormat::Float, 32) => reader.samples::<f32>()
            .map(|s| s.unwrap()) // Already in f32 format
            .collect(),
        _ => return Err("Unsupported sample format or bit depth".into()),
    };

    Ok((samples, sample_rate))
}
