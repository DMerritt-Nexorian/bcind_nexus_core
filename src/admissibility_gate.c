#include "../include/bcind_kernel.h"

bool evaluate_admissibility(TelemetryFrame* frame) {
    if (frame == NULL) {
        return false;
    }

    double total_signal_sq = 0.0;
    double total_noise_sq = 0.0;

    for (int i = 0; i < CHANNELS; i++) {
        total_signal_sq += frame->raw_signal_uv[i] * frame->raw_signal_uv[i];
        total_noise_sq += frame->noise_floor_uv[i] * frame->noise_floor_uv[i];
    }

    double rms_signal = sqrt(total_signal_sq / CHANNELS);
    double rms_noise = sqrt(total_noise_sq / CHANNELS);

    if (rms_noise < 1e-9) {
        frame->current_snr_db = 100.0; // Prevent divide by zero on zero noise
    } else {
        frame->current_snr_db = 20.0 * log10(rms_signal / rms_noise);
    }

    if (frame->current_snr_db < MIN_VIABLE_SNR_DB) {
        fprintf(stderr, "[ADMISSIBILITY] Gate Rejected: SNR %.2f dB below threshold %.2f dB.\n",
                frame->current_snr_db, MIN_VIABLE_SNR_DB);
        return false;
    }

    return true;
}
