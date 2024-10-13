fn delay_effect(samples: &mut [f32], sample_rate: f32, delay_time_ms: f32, feedback: f32) {
    let delay_samples = (sample_rate * delay_time_ms / 1000.0) as usize;
    let mut delay_buffer = vec![0.0; delay_samples];
    let mut delay_index = 0;

    for sample in samples.iter_mut() {
        let delayed_sample = delay_buffer[delay_index];
        let new_sample = *sample + delayed_sample * feedback;
        delay_buffer[delay_index] = new_sample;
        *sample = delayed_sample;
        delay_index = (delay_index + 1) % delay_samples;
    }
}

// Reverb using multiple delay lines, comb filters
fn reverb_effect(samples: &mut [f32], sample_rate: f32, room_size: f32, damping: f32) {
    let delay_times = [29, 37, 41, 43]; // Prime numbers for delay lengths
    let mut delay_lines: Vec<Vec<f32>> = delay_times
        .iter()
        .map(|&t| vec![0.0; (sample_rate * t as f32 / 1000.0) as usize])
        .collect();
    let mut indices = vec![0; delay_times.len()];

    for sample in samples.iter_mut() {
        let mut reverberated = 0.0;
        for (i, delay_line) in delay_lines.iter_mut().enumerate() {
            let index = indices[i];
            let delayed_sample = delay_line[index];
            delay_line[index] = *sample + delayed_sample * damping;
            reverberated += delayed_sample;
            indices[i] = (index + 1) % delay_line.len();
        }
        *sample = *sample * (1.0 - room_size) + reverberated * room_size;
    }
}

fn compressor(
    samples: &mut [f32],
    threshold: f32,
    ratio: f32,
    attack_ms: f32,
    release_ms: f32,
    sample_rate: f32,
) {
    let attack_coeff = (-1.0 / (attack_ms * 0.001 * sample_rate)).exp();
    let release_coeff = (-1.0 / (release_ms * 0.001 * sample_rate)).exp();

    let mut gain = 1.0;

    for sample in samples.iter_mut() {
        let input_level = sample.abs();
        let over_threshold = input_level > threshold;

        if over_threshold {
            let compressed_gain = threshold + (input_level - threshold) / ratio;
            gain = attack_coeff * (gain - compressed_gain) + compressed_gain;
        } else {
            gain = release_coeff * (gain - 1.0) + 1.0;
        }

        *sample *= gain;
    }
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

fn remove_dc_offset(samples: &mut [f32]) {
    let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
    for sample in samples.iter_mut() {
        *sample -= mean;
    }
}

// Soft clipping distortion with harmonic content
fn distortion(samples: &mut [f32], gain: f32, threshold: f32) {
    for sample in samples.iter_mut() {
        *sample *= gain;
        if *sample > threshold {
            *sample = threshold + (1.0 - threshold) * ((*sample - threshold) / (1.0 - threshold)).tanh();
        } else if *sample < -threshold {
            *sample = -threshold + (-1.0 + threshold) * ((*sample + threshold) / (-1.0 + threshold)).tanh();
        }
    }
}

fn tremolo_effect(samples: &mut [f32], sample_rate: f32, rate_hz: f32, depth: f32) {
    let lfo_increment = 2.0 * std::f32::consts::PI * rate_hz / sample_rate;
    let mut lfo_phase = 0.0;

    for sample in samples.iter_mut() {
        let lfo_value = (lfo_phase.sin() * 0.5 + 0.5) * depth + (1.0 - depth);
        *sample *= lfo_value;
        lfo_phase = (lfo_phase + lfo_increment) % (2.0 * std::f32::consts::PI);
    }
}


fn flanger_effect(
    samples: &mut [f32],
    sample_rate: f32,
    depth_ms: f32,
    rate_hz: f32,
    feedback: f32,
    mix: f32,
) {
    let max_delay_samples = (sample_rate * depth_ms / 1000.0) as usize;
    let mut delay_buffer = vec![0.0; max_delay_samples];
    let mut delay_index = 0;

    let lfo_increment = 2.0 * std::f32::consts::PI * rate_hz / sample_rate;
    let mut lfo_phase = 0.0;

    for sample in samples.iter_mut() {
        let lfo_value = (lfo_phase.sin() * 0.5 + 0.5);
        let current_delay = (lfo_value * max_delay_samples as f32) as usize;

        let delayed_sample = delay_buffer[(delay_index + max_delay_samples - current_delay) % max_delay_samples];
        let new_sample = *sample + delayed_sample * feedback;

        delay_buffer[delay_index] = new_sample;
        *sample = *sample * (1.0 - mix) + delayed_sample * mix;

        delay_index = (delay_index + 1) % max_delay_samples;
        lfo_phase = (lfo_phase + lfo_increment) % (2.0 * std::f32::consts::PI);
    }
}

fn stereo_panning(samples: &[f32], pan: f32) -> (Vec<f32>, Vec<f32>) {
    let left_gain = ((1.0 - pan) * std::f32::consts::FRAC_PI_2).cos();
    let right_gain = (pan * std::f32::consts::FRAC_PI_2).cos();

    let left_channel: Vec<f32> = samples.iter().map(|&s| s * left_gain).collect();
    let right_channel: Vec<f32> = samples.iter().map(|&s| s * right_gain).collect();

    (left_channel, right_channel)
}

fn noise_gate(
    samples: &mut [f32],
    threshold: f32,
    sample_rate: f32,
    attack_ms: f32,
    release_ms: f32,
) {
    let attack_coeff = (-1.0 / (attack_ms * 0.001 * sample_rate)).exp();
    let release_coeff = (-1.0 / (release_ms * 0.001 * sample_rate)).exp();

    let mut gain = 1.0;

    for sample in samples.iter_mut() {
        let input_level = sample.abs();

        if input_level < threshold {
            gain = attack_coeff * (gain - 0.0) + 0.0;
        } else {
            gain = release_coeff * (gain - 1.0) + 1.0;
        }

        *sample *= gain;
    }
}

