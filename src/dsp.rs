// Examples of DSP functions
// High-pass filter constants (sample rate dependent)
const SAMPLE_RATE: f32 = 44100.0;  // Assuming 44.1kHz sample rate
const HPF_CUTOFF_FREQ: f32 = 100.0;  // Cutoff frequency for the high-pass filter

/// A simple dereverb function to process an audio vector and reduce reverberation.
///
/// # Arguments
///
/// * `input` - A reference of vector of `f32` values representing the audio signal.
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
/// * `sample` - A reference of single f32 audio sample.
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

fn convolve(signal: &[f32], impulse_response: &[f32]) -> Vec<f32> {
    let n = signal.len();
    let m = impulse_response.len();
    let mut output = vec![0.0; n + m - 1];

    for i in 0..n {
        for j in 0..m {
            output[i + j] += signal[i] * impulse_response[j];
        }
    }

    // Resulting signal length is n + m - 1.
    output
}

enum WindowType {
    Hamming,
    Hann,
    Blackman,
}

fn apply_window(samples: &mut [f32], window_type: WindowType) {
    let n = samples.len();
    for (i, sample) in samples.iter_mut().enumerate() {
        let multiplier = match window_type {
            WindowType::Hamming => {
                0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos()
            }
            WindowType::Hann => {
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos())
            }
            WindowType::Blackman => {
                0.42 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos()
                    + 0.08 * (4.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos()
            }
        };
        *sample *= multiplier;
    }
}

fn envelope_detection(samples: &[f32]) -> Vec<f32> {
    let mut envelope = Vec::with_capacity(samples.len());
    let mut previous = 0.0;
    let alpha = 0.1; // Smoothing factor (adjustable)

    for &sample in samples {
        let rectified = sample.abs();
        let current = alpha * rectified + (1.0 - alpha) * previous;
        envelope.push(current);
        previous = current;
    }

    envelope
}

fn calculate_rms(samples: &[f32]) -> f32 {
    let sum_squares: f32 = samples.iter().map(|&x| x * x).sum();
    let mean_square = sum_squares / samples.len() as f32;
    mean_square.sqrt()
}

fn peak_detection(samples: &[f32]) -> f32 {
    samples.iter().map(|&x| x.abs()).fold(0.0, f32::max)
}

// Resample based on linear interpolation
fn resample(samples: &[f32], original_rate: f32, target_rate: f32) -> Vec<f32> {
    let resample_ratio = target_rate / original_rate;
    let new_length = ((samples.len() as f32) * resample_ratio).round() as usize;
    let mut resampled = Vec::with_capacity(new_length);

    for i in 0..new_length {
        let src_index = i as f32 / resample_ratio;
        let index_floor = src_index.floor() as usize;
        let index_ceil = (index_floor + 1).min(samples.len() - 1);
        let weight = src_index - index_floor as f32;

        let interpolated = samples[index_floor] * (1.0 - weight) + samples[index_ceil] * weight;
        resampled.push(interpolated);
    }

    resampled
}

fn normalize(samples: &mut [f32]) {
    if let Some(max_amplitude) = samples.iter().map(|&x| x.abs()).fold(None, |max, x| {
        Some(if let Some(current_max) = max {
            if x > current_max {
                x
            } else {
                current_max
            }
        } else {
            x
        })
    }) {
        if max_amplitude > 0.0 {
            let normalizing_factor = 1.0 / max_amplitude;
            for sample in samples.iter_mut() {
                *sample *= normalizing_factor;
            }
        }
    }
}


// Separating or combining mid and side signals:
fn mid_side_encode(left_channel: &[f32], right_channel: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mid: Vec<f32> = left_channel
        .iter()
        .zip(right_channel)
        .map(|(&l, &r)| (l + r) * 0.5)
        .collect();
    let side: Vec<f32> = left_channel
        .iter()
        .zip(right_channel)
        .map(|(&l, &r)| (l - r) * 0.5)
        .collect();
    (mid, side)
}

fn mid_side_decode(mid: &[f32], side: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let left_channel: Vec<f32> = mid.iter().zip(side).map(|(&m, &s)| m + s).collect();
    let right_channel: Vec<f32> = mid.iter().zip(side).map(|(&m, &s)| m - s).collect();
    (left_channel, right_channel)
}

