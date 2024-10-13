struct EQBand {
    frequency: f32,
    gain_db: f32,
    q_factor: f32,
}

fn equalizer(samples: &mut [f32], sample_rate: f32, bands: &[EQBand]) {
    let mut filters: Vec<BiquadFilter> = bands
        .iter()
        .map(|band| BiquadFilter::new_peaking_eq(sample_rate, band.frequency, band.q_factor, band.gain_db))
        .collect();

    for sample in samples.iter_mut() {
        let mut processed_sample = *sample;
        for filter in filters.iter_mut() {
            processed_sample = filter.process_sample(processed_sample);
        }
        *sample = processed_sample;
    }
}

impl BiquadFilter {
    fn new_peaking_eq(sample_rate: f32, freq: f32, q: f32, gain_db: f32) -> Self {
        let omega = 2.0 * std::f32::consts::PI * freq / sample_rate;
        let alpha = omega.sin() / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * omega.cos();
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * omega.cos();
        let a2 = 1.0 - alpha / a;

        Self {
            a0: b0 / a0,
            a1: b1 / a0,
            a2: b2 / a0,
            b1: a1 / a0,
            b2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }
}

