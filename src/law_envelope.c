#include "../include/bcind_kernel.h"

bool check_law_envelope(const TelemetryFrame* frame, const char* jurisdiction) {
    if (frame == NULL || jurisdiction == NULL) return false;

    // Enforce biophysical boundaries across all 32 channels
    for (int i = 0; i < CHANNELS; i++) {
        if (frame->channel_impedance_kohm[i] < 0.0 || frame->channel_impedance_kohm[i] > MAX_CONTACT_IMPEDANCE_KOHM) {
            fprintf(stderr, "[LAW ENVELOPE FAULT] Channel %d impedance (%.2f kOhm) violates safety bound [0, %.1f kOhm]\n",
                    i + 1, frame->channel_impedance_kohm[i], MAX_CONTACT_IMPEDANCE_KOHM);
            return false;
        }
    }

    if (strcmp(jurisdiction, "ISO13485") == 0 || strcmp(jurisdiction, "IEC62304") == 0) {
        // Medical device software compliance requires hardware contact validity flag
        if (!frame->hardware_contact_valid) {
            fprintf(stderr, "[LAW ENVELOPE FAULT] Hardware contact validity flag is FALSE under %s jurisdiction.\n", jurisdiction);
            return false;
        }
    }
    return true;
}
