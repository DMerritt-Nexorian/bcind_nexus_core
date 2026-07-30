#include "../include/bcind_kernel.h"

static void print_usage(const char* prog) {
    printf("Usage: %s [OPTIONS]\n", prog);
    printf("Options:\n");
    printf("  --impedance <val>   Set scalp contact impedance for all channels in kOhm (default: 45.0)\n");
    printf("  --signal <val>      Set microvolt signal amplitude in uV (default: 12.5)\n");
    printf("  --noise <val>       Set microvolt noise amplitude in uV (default: 25.0)\n");
    printf("  --jurisdiction <str>Set jurisdiction code [ISO13485|IEC62304|DIN3105] (default: IEC62304)\n");
    printf("  --config <path>     Path to governance JSON config (default: config/governance_state.json)\n");
    printf("  --audit <path>      Path to audit log JSON output (default: audit_export.json)\n");
    printf("  --help              Display this help message\n");
}

int main(int argc, char** argv) {
    double input_impedance = 45.0;
    double input_signal = 12.5;
    double input_noise = 25.0;
    char jurisdiction[16] = "IEC62304";
    char config_path[128] = "config/governance_state.json";
    char audit_path[128] = "audit_export.json";

    // Parse dynamic CLI arguments
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--impedance") == 0 && i + 1 < argc) input_impedance = atof(argv[++i]);
        else if (strcmp(argv[i], "--signal") == 0 && i + 1 < argc) input_signal = atof(argv[++i]);
        else if (strcmp(argv[i], "--noise") == 0 && i + 1 < argc) input_noise = atof(argv[++i]);
        else if (strcmp(argv[i], "--jurisdiction") == 0 && i + 1 < argc) strncpy(jurisdiction, argv[++i], sizeof(jurisdiction)-1);
        else if (strcmp(argv[i], "--config") == 0 && i + 1 < argc) strncpy(config_path, argv[++i], sizeof(config_path)-1);
        else if (strcmp(argv[i], "--audit") == 0 && i + 1 < argc) strncpy(audit_path, argv[++i], sizeof(audit_path)-1);
        else if (strcmp(argv[i], "--help") == 0) {
            print_usage(argv[0]);
            return 0;
        }
    }

    printf("========================================================\n");
    printf("   BCIND NEXUS-GENESIS KERNEL v%s\n", SYSTEM_VERSION);
    printf("   Non-Invasive Brain-Computer Interface Neural Decoder\n");
    printf("========================================================\n\n");

    if (!verify_immutable_core()) {
        execute_reflex_action("Immutable Core Integrity Check Failed");
        return 1;
    }

    GovernanceState gov_state;
    if (load_governance_state(&gov_state, config_path) != 0) {
        fprintf(stderr, "[ERROR] Failed to load or create governance state at %s\n", config_path);
        return 1;
    }

    if (strlen(jurisdiction) > 0) {
        strncpy(gov_state.jurisdiction, jurisdiction, sizeof(gov_state.jurisdiction)-1);
    }

    TelemetryFrame frame;
    frame.hardware_contact_valid = true;
    for (int i = 0; i < CHANNELS; i++) {
        frame.channel_impedance_kohm[i] = input_impedance;
        frame.raw_signal_uv[i] = input_signal;
        frame.noise_floor_uv[i] = input_noise;
    }

    printf("[INIT] System state loaded. Sequence: %lu | Jurisdiction: %s\n",
           (unsigned long)gov_state.audit_sequence, gov_state.jurisdiction);

    if (!ceal_enforce_policy(&frame, &gov_state)) {
        log_audit_event(audit_path, "KERNEL_EXEC", "REJECTED_CEAL", 0.0, gov_state.audit_sequence);
        return 1;
    }

    if (!check_law_envelope(&frame, gov_state.jurisdiction)) {
        execute_reflex_action("Law Envelope Boundary Breach");
        log_audit_event(audit_path, "KERNEL_EXEC", "REJECTED_LAW_ENVELOPE", 0.0, gov_state.audit_sequence);
        return 1;
    }

    if (!evaluate_admissibility(&frame)) {
        execute_reflex_action("Admissibility Gate Failed: Insufficient SNR");
        log_audit_event(audit_path, "KERNEL_EXEC", "REJECTED_ADMISSIBILITY", frame.current_snr_db, gov_state.audit_sequence);
        return 1;
    }

    printf("[SUCCESS] Telemetry frame admissible!\n");
    printf("          Calculated SNR: %.2f dB | Contact Impedance: %.2f kOhm\n",
           frame.current_snr_db, input_impedance);

    gov_state.audit_sequence++;
    if (save_governance_state(&gov_state, config_path) == 0) {
        printf("[STATE] Persisted sequence %lu to %s\n", (unsigned long)gov_state.audit_sequence, config_path);
    }

    log_audit_event(audit_path, "KERNEL_EXEC", "PASS", frame.current_snr_db, gov_state.audit_sequence);
    return 0;
}
