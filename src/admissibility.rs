use crate::law_envelope::{CHANNELS, TelemetryFrame};

pub const MIN_VIABLE_SNR_DB: f64 = -10.0;

pub fn evaluate_admissibility(frame: &mut TelemetryFrame) -> bool {
    let mut total_signal_sq = 0.0;
    let mut total_noise_sq = 0.0;

    for i in 0..CHANNELS {
        total_signal_sq += frame.raw_signal_uv[i] * frame.raw_signal_uv[i];
        total_noise_sq += frame.noise_floor_uv[i] * frame.noise_floor_uv[i];
    }

    let rms_signal = (total_signal_sq / CHANNELS as f64).sqrt();
    let rms_noise = (total_noise_sq / CHANNELS as f64).sqrt();

    if rms_noise < 1e-9 {
        frame.current_snr_db = 100.0; // Prevent division by zero
    } else {
        frame.current_snr_db = 20.0 * (rms_signal / rms_noise).log10();
    }

    if frame.current_snr_db < MIN_VIABLE_SNR_DB {
        log::error!(
            "[ADMISSIBILITY] Gate Rejected: SNR {:.2} dB below threshold {:.2} dB.",
            frame.current_snr_db,
            MIN_VIABLE_SNR_DB
        );
        false
    } else {
        true
    }
}
