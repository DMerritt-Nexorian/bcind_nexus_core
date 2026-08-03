use bcind_core::admissibility::{MIN_VIABLE_SNR_DB, evaluate_admissibility};
use bcind_core::dsp::{
    BiquadFilter, apply_common_average_reference, clean_signal_artifacts, compute_eeg_band_powers,
};
use bcind_core::law_envelope::{CHANNELS, TelemetryFrame, check_law_envelope};

/// Generates a synthetic EEG signal chunk.
/// Synthesizes alpha, beta, gamma, theta, delta waves, SSVEP, and optional movement artifacts.
#[allow(clippy::too_many_arguments)]
pub fn generate_mock_eeg_chunk(
    fs: f64,
    seconds: f64,
    alpha_amp: f64,
    beta_amp: f64,
    gamma_amp: f64,
    theta_amp: f64,
    delta_amp: f64,
    ssvep_freq: f64,
    ssvep_amp: f64,
    artifact_amp: f64,
) -> Vec<f64> {
    let num_samples = (fs * seconds) as usize;
    let mut data = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f64 / fs;
        // EEG standard rhythms
        let delta = delta_amp * (2.0 * std::f64::consts::PI * 2.5 * t).sin(); // 2.5 Hz
        let theta = theta_amp * (2.0 * std::f64::consts::PI * 6.0 * t).sin(); // 6.0 Hz
        let alpha = alpha_amp * (2.0 * std::f64::consts::PI * 10.0 * t).sin(); // 10.0 Hz
        let beta = beta_amp * (2.0 * std::f64::consts::PI * 20.0 * t).sin(); // 20.0 Hz
        let gamma = gamma_amp * (2.0 * std::f64::consts::PI * 40.0 * t).sin(); // 40.0 Hz

        // SSVEP frequency (e.g. 15 Hz flickering response)
        let ssvep = ssvep_amp * (2.0 * std::f64::consts::PI * ssvep_freq * t).sin();

        // Artifact (e.g., sudden movement spike at t=1.0 seconds)
        let artifact = if artifact_amp > 0.0 && (t - 1.0).abs() < 0.05 {
            artifact_amp * (2.0 * std::f64::consts::PI * 15.0 * t).sin()
        } else {
            0.0
        };

        data.push(delta + theta + alpha + beta + gamma + ssvep + artifact);
    }

    data
}

#[test]
fn test_mock_signal_generation_and_spectral_power() {
    let fs = 250.0; // 250 Hz sampling rate (standard for OpenBCI Cyton)
    let seconds = 4.0; // 1024 samples

    // Generate pure 10 Hz alpha wave chunk
    let alpha_wave = generate_mock_eeg_chunk(fs, seconds, 15.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    // Compute spectral power
    let powers = compute_eeg_band_powers(&alpha_wave, fs);

    println!("Alpha Power: {:.2}", powers.alpha);
    println!("Beta Power: {:.2}", powers.beta);

    assert!(powers.alpha > 0.0);
    assert!(powers.alpha > powers.beta);
}

#[test]
fn test_common_average_reference() {
    let channels_data = vec![10.0, 11.0, 9.0, 10.0, 30.0]; // channel 5 has high common mode noise
    let car_filtered = apply_common_average_reference(&channels_data);

    let sum_filtered: f64 = car_filtered.iter().sum();
    // Sum of common average referenced signals should be mathematically 0.0
    assert!((sum_filtered.abs()) < 1e-9);
}

#[test]
fn test_artifact_removal() {
    // Generate 200 healthy baseline samples
    let mut signal = vec![1.2; 200];
    // Add one massive muscle artifact spike of 500 uV at sample index 100
    signal[100] = 500.0;

    clean_signal_artifacts(&mut signal, 2.0);

    // Check that artifact is successfully soft-clipped to a much lower, safer value
    println!("Clipped spike value: {:.2}", signal[100]);
    assert!(signal[100] < 100.0);
}

#[test]
fn test_biquad_filtering() {
    let fs = 250.0;
    let mut lp = BiquadFilter::lowpass(30.0, fs);

    // Verify a step response settles
    let mut out = 0.0;
    for _ in 0..100 {
        out = lp.process_sample(10.0);
    }
    assert!((out - 10.0).abs() < 1.0);
}

#[test]
fn test_law_envelope_and_admissibility_rejections() {
    // 1. Valid telemetry frame must pass
    let mut valid_frame = TelemetryFrame {
        hardware_contact_valid: true,
        channel_impedance_kohm: [45.0; CHANNELS],
        raw_signal_uv: [15.0; CHANNELS],
        noise_floor_uv: [5.0; CHANNELS],
        current_snr_db: 0.0,
    };

    assert!(check_law_envelope(&valid_frame, "IEC62304"));
    assert!(evaluate_admissibility(&mut valid_frame));
    assert!(valid_frame.current_snr_db > MIN_VIABLE_SNR_DB);

    // 2. Telemetry frame with impedance exceeding 150 kΩ must fail check_law_envelope
    let mut high_impedance_frame = TelemetryFrame::default();
    high_impedance_frame.channel_impedance_kohm[12] = 155.0; // Exceeds 150.0 Limit
    assert!(!check_law_envelope(&high_impedance_frame, "IEC62304"));

    // 3. Telemetry frame with non-physical impedance (<= 0.0) must fail
    let mut non_physical_frame = TelemetryFrame::default();
    non_physical_frame.channel_impedance_kohm[3] = -5.0;
    assert!(!check_law_envelope(&non_physical_frame, "IEC62304"));

    // 4. Telemetry frame with very poor SNR must fail admissibility
    let mut noisy_frame = TelemetryFrame {
        hardware_contact_valid: true,
        channel_impedance_kohm: [30.0; CHANNELS],
        raw_signal_uv: [1.0; CHANNELS],
        noise_floor_uv: [50.0; CHANNELS], // noisy background
        current_snr_db: 0.0,
    };
    assert!(!evaluate_admissibility(&mut noisy_frame));
}
