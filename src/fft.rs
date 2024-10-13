use rustfft::{FftPlanner, num_complex::Complex};

fn fft(samples: &[f32]) -> Vec<Complex<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();
    fft.process(&mut buffer);

    buffer
}

fn ifft(frequency_data: &[Complex<f32>]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let ifft = planner.plan_fft_inverse(frequency_data.len());

    let mut buffer = frequency_data.to_owned();
    ifft.process(&mut buffer);

    // Normalize the output
    buffer.iter().map(|c| c.re / frequency_data.len() as f32).collect()
}
