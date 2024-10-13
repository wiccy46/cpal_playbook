// First order low-pass IIR filter
// y[n] = y[n-1] + α * (x[n] - y[n-1]).
fn low_pass_filter(samples: &mut [f32], sample_rate: f32, cutoff_freq: f32) {
    // Time constant
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_freq);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);

    let mut previous = samples[0];
    for sample in samples.iter_mut() {
        previous = previous + alpha * (*sample - previous);
        // Because sample is a reference, we need to dereference it to actually 
        // update the value in the slice.
        *sample = previous;
    }
}

// First order high-pass IIR filter
// y[n] = α * (y[n-1] + x[n] - x[n-1])
fn high_pass_filter(samples: &mut [f32], sample_rate: f32, cutoff_freq: f32) {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_freq);
    let dt = 1.0 / sample_rate;
    let alpha = rc / (rc + dt);

    let mut previous_input = samples[0];
    let mut previous_output = samples[0];

    for sample in samples.iter_mut() {
        let current_input = *sample;
        *sample = alpha * (previous_output + current_input - previous_input);
        previous_output = *sample;
        previous_input = current_input;
    }
}

// Biquad
// Exaple usage:
// fn low_pass_filter(samples: &mut [f32], sample_rate: f32, cutoff_freq: f32, q_factor: f32) {
//    let mut filter = BiquadFilter::new_lowpass(sample_rate, cutoff_freq, q_factor);
//
//    for sample in samples.iter_mut() {
//        *sample = filter.process_sample(*sample);
//    }
//}

struct BiquadFilter {
    // Feedforward coefficients
    b0: f32,
    b1: f32,
    b2: f32,
    // Feedback coefficients
    a1: f32,
    a2: f32,
    // Delayed samples (for processing)
    z1: f32,
    z2: f32,
}

impl BiquadFilter {
    /// Processes a single sample through the filter.
    fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.z1 + self.b2 * self.z2
            - self.a1 * self.z1 - self.a2 * self.z2;
        self.z2 = self.z1;
        self.z1 = output;
        output
    }

    /// Creates a low-pass filter.
    fn new_lowpass(sample_rate: f32, cutoff_freq: f32, q_factor: f32) -> Self {
        let omega = 2.0 * std::f32::consts::PI * cutoff_freq / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q_factor);

        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }

    /// Creates a high-pass filter.
    fn new_highpass(sample_rate: f32, cutoff_freq: f32, q_factor: f32) -> Self {
        let omega = 2.0 * std::f32::consts::PI * cutoff_freq / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q_factor);

        let b0 = (1.0 + cos_omega) / 2.0;
        let b1 = -(1.0 + cos_omega);
        let b2 = (1.0 + cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }

    /// Creates a low-shelf filter.
    fn new_lowshelf(sample_rate: f32, cutoff_freq: f32, gain_db: f32, slope: f32) -> Self {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let omega = 2.0 * std::f32::consts::PI * cutoff_freq / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / 2.0 * ((a + 1.0 / a) * (1.0 / slope - 1.0) + 2.0).sqrt();
        let beta = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_omega + beta);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_omega - beta);
        let a0 = (a + 1.0) + (a - 1.0) * cos_omega + beta;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) + (a - 1.0) * cos_omega - beta;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }

    /// Creates a high-shelf filter.
    fn new_highshelf(sample_rate: f32, cutoff_freq: f32, gain_db: f32, slope: f32) -> Self {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let omega = 2.0 * std::f32::consts::PI * cutoff_freq / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / 2.0 * ((a + 1.0 / a) * (1.0 / slope - 1.0) + 2.0).sqrt();
        let beta = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_omega + beta);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_omega - beta);
        let a0 = (a + 1.0) - (a - 1.0) * cos_omega + beta;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) - (a - 1.0) * cos_omega - beta;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }

    /// Creates a peaking EQ filter.
    fn new_peaking_eq(sample_rate: f32, freq: f32, q_factor: f32, gain_db: f32) -> Self {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let omega = 2.0 * std::f32::consts::PI * freq / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q_factor);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_omega;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }
}

