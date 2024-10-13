// Examples of DSP functions
// High-pass filter constants (sample rate dependent)
const SAMPLE_RATE: f32 = 44100.0;  // Assuming 44.1kHz sample rate
const HPF_CUTOFF_FREQ: f32 = 100.0;  // Cutoff frequency for the high-pass filter

/// A simple dereverb function to process an audio vector and reduce reverberation.
///
/// # Arguments
///
/// * `input` - A vector of `f32` values representing the audio signal.
///
/// # Returns
///
/// A new `f32` vector with reduced reverberation.
pub fn dereverb(input: &Vec<f32>) -> Vec<f32> {
    let mut output = Vec::with_capacity(input.len());
    
    // High-pass filter state variables
    let mut prev_input = 0.0;
    let mut prev_output = 0.0;

    // Coefficients for the high-pass filter (1st order)
    let rc = 1.0 / (HPF_CUTOFF_FREQ * 2.0 * std::f32::consts::PI);
    let dt = 1.0 / SAMPLE_RATE;
    let alpha = dt / (rc + dt);

    for &sample in input.iter() {
        // Apply a simple high-pass filter to remove low-frequency reverberation
        let filtered = alpha * (prev_output + sample - prev_input);

        // Update state for the next iteration
        prev_input = sample;
        prev_output = filtered;

        // Apply an early reflection suppression (simple attenuation)
        let attenuation = early_reflection_suppression(&filtered);
        
        // Push the processed sample to the output
        output.push(attenuation);
    }

    output
}

/// Apply a simple attenuation to reduce late reverberations.
/// 
/// This function simulates reducing the strength of reflections
/// by dampening the signal after a certain threshold.
/// 
/// # Arguments
///
/// * `sample` - A single f32 audio sample.
///
/// # Returns
///
/// The attenuated sample.
fn early_reflection_suppression(sample: &f32) -> f32 {
    let reflection_threshold = 0.05; // Threshold for suppressing reflections
    let attenuation_factor = 0.8;    // Attenuation factor for late reflections

    // If the sample magnitude is smaller than the threshold, apply attenuation
    if sample.abs() < reflection_threshold {
        sample * attenuation_factor
    } else {
        sample
    }
}

