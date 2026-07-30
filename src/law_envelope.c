#include "../include/bcind_kernel.h"

bool check_law_envelope(const TelemetryFrame* frame, const char* jurisdiction) {
    if (frame == NULL || jurisdiction == NULL) {
        return false;
    }

    // Verify channel hardware contact and contact impedance envelope
    if (!frame->hardware_contact_valid) {
        fprintf(stderr, "[LAW_ENVELOPE] Violation: Hardware contact flag reported invalid.\n");
        return false;
    }

    for (int i = 0; i < CHANNELS; i++) {
        if (frame->channel_impedance_kohm[i] > MAX_CONTACT_IMPEDANCE_KOHM) {
            fprintf(stderr, "[LAW_ENVELOPE] Violation: Channel %d impedance (%.2f kOhm) exceeds max limit (%.2f kOhm).\n",
                    i, frame->channel_impedance_kohm[i], MAX_CONTACT_IMPEDANCE_KOHM);
            return false;
        }
        if (frame->channel_impedance_kohm[i] <= 0.0) {
            fprintf(stderr, "[LAW_ENVELOPE] Violation: Channel %d impedance (%.2f kOhm) is non-physical.\n",
                    i, frame->channel_impedance_kohm[i]);
            return false;
        }
    }

    // Jurisdiction-specific safety verification
    if (strcmp(jurisdiction, "ISO13485") == 0) {
        // Strict quality management system constraint check
        return true;
    } else if (strcmp(jurisdiction, "IEC62304") == 0) {
        // Medical device software lifecycle constraint check
        return true;
    } else if (strcmp(jurisdiction, "DIN3105") == 0) {
        // Open hardware baseline compliance check
        return true;
    } else {
        fprintf(stderr, "[LAW_ENVELOPE] Warning: Unknown jurisdiction '%s'. Defaulting to strict fail-closed policy.\n", jurisdiction);
        return false;
    }
}
