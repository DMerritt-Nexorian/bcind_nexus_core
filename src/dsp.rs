use rustfft::{FftPlanner, num_complex::Complex};

/// Biquad filter implementation (Direct Form II Transposed).
/// Used for real-time temporal filtering (bandpass, lowpass, highpass, or notch).
#[derive(Debug, Clone)]
pub struct BiquadFilter {
    // Filter coefficients
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub a1: f64,
    pub a2: f64,
    // Filter state
    pub s1: f64,
    pub s2: f64,
}

impl BiquadFilter {
    /// Creates a new Biquad filter with specified coefficients.
    /// Note: Coefficients should be normalized by a0 (so a0 = 1.0).
    pub fn new(b0: f64, b1: f64, b2: f64, a1: f64, a2: f64) -> Self {
        Self {
            b0,
            b1,
            b2,
            a1,
            a2,
            s1: 0.0,
            s2: 0.0,
        }
    }

    /// Resets the filter internal state.
    pub fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }

    /// Processes a single input sample through the biquad filter.
    #[inline]
    pub fn process_sample(&mut self, input: f64) -> f64 {
        let output = self.b0 * input + self.s1;
        self.s1 = self.b1 * input - self.a1 * output + self.s2;
        self.s2 = self.b2 * input - self.a2 * output;
        output
    }

    /// Creates a 2nd-order low-pass Butterworth filter coefficient set.
    /// `fc` is cutoff frequency, `fs` is sampling rate.
    pub fn lowpass(fc: f64, fs: f64) -> Self {
        let wd = 2.0 * std::f64::consts::PI * fc;
        let wa = (2.0 * fs) * (wd / (2.0 * fs)).tan(); // Bilinear transform pre-warping
        let g = wa / (2.0 * fs);
        let g_sq = g * g;
        let sqrt2 = 2.0_f64.sqrt();
        let denom = 1.0 + sqrt2 * g + g_sq;

        let b0 = g_sq / denom;
        let b1 = 2.0 * b0;
        let b2 = b0;
        let a1 = (2.0 * g_sq - 2.0) / denom;
        let a2 = (1.0 - sqrt2 * g + g_sq) / denom;

        Self::new(b0, b1, b2, a1, a2)
    }

    /// Creates a 2nd-order high-pass Butterworth filter coefficient set.
    pub fn highpass(fc: f64, fs: f64) -> Self {
        let wd = 2.0 * std::f64::consts::PI * fc;
        let wa = (2.0 * fs) * (wd / (2.0 * fs)).tan();
        let g = wa / (2.0 * fs);
        let g_sq = g * g;
        let sqrt2 = 2.0_f64.sqrt();
        let denom = 1.0 + sqrt2 * g + g_sq;

        let b0 = 1.0 / denom;
        let b1 = -2.0 * b0;
        let b2 = b0;
        let a1 = (2.0 * g_sq - 2.0) / denom;
        let a2 = (1.0 - sqrt2 * g + g_sq) / denom;

        Self::new(b0, b1, b2, a1, a2)
    }

    /// Creates a 2nd-order band-stop (notch) filter centered at `f0` with quality factor `q`.
    pub fn notch(f0: f64, q: f64, fs: f64) -> Self {
        let w0 = 2.0 * std::f64::consts::PI * f0 / fs;
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();

        let b0 = 1.0;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        Self::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }
}

/// Applies Common Average Reference (CAR) to reduce global common-mode interference
/// across multi-channel biophysical EEG data.
/// Returns a new vector containing spatial-filtered values.
pub fn apply_common_average_reference(channels: &[f64]) -> Vec<f64> {
    if channels.is_empty() {
        return Vec::new();
    }
    let sum: f64 = channels.iter().sum();
    let mean = sum / channels.len() as f64;
    channels.iter().map(|&x| x - mean).collect()
}

/// Structural container for calculated EEG spectral power across standard physiological bands.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EegBandsPower {
    pub delta: f64, // 1.0 - 4.0 Hz
    pub theta: f64, // 4.0 - 8.0 Hz
    pub alpha: f64, // 8.0 - 13.0 Hz
    pub beta: f64,  // 13.0 - 30.0 Hz
    pub gamma: f64, // 30.0 - 100.0 Hz
}

/// Computes the Fast Fourier Transform (FFT) of a channel data buffer,
/// and returns the integrated power inside standard EEG frequency bands.
/// `buffer` must have a length of a power of 2 (e.g. 1024 or 2048) for computational efficiency.
pub fn compute_eeg_band_powers(buffer: &[f64], fs: f64) -> EegBandsPower {
    let n = buffer.len();
    if n == 0 {
        return EegBandsPower::default();
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);

    // Prepare complex input with a Hanning window to prevent spectral leakage
    let mut complex_buffer: Vec<Complex<f64>> = buffer
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            let window =
                0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64).cos());
            Complex::new(val * window, 0.0)
        })
        .collect();

    fft.process(&mut complex_buffer);

    // Compute single-sided Power Spectral Density (PSD)
    let mut psd = vec![0.0; n / 2 + 1];
    let df = fs / n as f64; // Frequency resolution

    for i in 0..=n / 2 {
        let mag = complex_buffer[i].norm();
        // Scaling factor for power to satisfy Parseval's theorem
        let scale = if i == 0 || i == n / 2 { 1.0 } else { 2.0 };
        psd[i] = scale * (mag * mag) / (n as f64 * fs);
    }

    let mut bands = EegBandsPower::default();

    for (i, &power) in psd.iter().enumerate() {
        let freq = i as f64 * df;
        if (1.0..4.0).contains(&freq) {
            bands.delta += power * df;
        } else if (4.0..8.0).contains(&freq) {
            bands.theta += power * df;
        } else if (8.0..13.0).contains(&freq) {
            bands.alpha += power * df;
        } else if (13.0..30.0).contains(&freq) {
            bands.beta += power * df;
        } else if (30.0..=100.0).contains(&freq) {
            bands.gamma += power * df;
        }
    }

    bands
}

/// Identifies and suppresses high-amplitude non-neural biophysical artifacts (e.g. EOG ocular or EMG facial muscle).
/// Applies soft-thresholding on standard deviation of the signal segment.
pub fn clean_signal_artifacts(signal: &mut [f64], threshold_multiplier: f64) {
    let n = signal.len();
    if n < 2 {
        return;
    }

    let mean: f64 = signal.iter().sum::<f64>() / n as f64;
    let variance: f64 = signal.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    let std_dev = variance.sqrt();

    let threshold = threshold_multiplier * std_dev;

    // Zero-out or clip artifacts exceeding the statistical boundaries
    for val in signal.iter_mut() {
        let diff = (*val - mean).abs();
        if diff > threshold {
            // Soft interpolation back towards the local mean
            *val = mean + (*val - mean).signum() * threshold;
        }
    }
}
