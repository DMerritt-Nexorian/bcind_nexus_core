#include "../include/bcind_kernel.h"

int load_governance_state(GovernanceState* state, const char* filepath) {
    if (state == NULL || filepath == NULL) {
        return -1;
    }

    FILE* fp = fopen(filepath, "r");
    if (!fp) {
        // Defaults if file doesn't exist yet
        state->state_id = 1;
        state->execution_permitted = true;
        state->immutable_lock = true;
        strncpy(state->jurisdiction, "IEC62304", sizeof(state->jurisdiction)-1);
        state->audit_sequence = 1000;
        return save_governance_state(state, filepath);
    }

    char buffer[512];
    size_t bytes_read = fread(buffer, 1, sizeof(buffer)-1, fp);
    fclose(fp);
    buffer[bytes_read] = '\0';

    // Basic JSON state extraction
    state->state_id = 1;
    state->execution_permitted = (strstr(buffer, "\"execution_permitted\": true") != NULL || strstr(buffer, "\"execution_permitted\":true") != NULL);
    state->immutable_lock = (strstr(buffer, "\"immutable_lock\": true") != NULL || strstr(buffer, "\"immutable_lock\":true") != NULL);
    
    char* jur_ptr = strstr(buffer, "\"jurisdiction\":");
    if (jur_ptr) {
        char jur_val[16] = {0};
        if (sscanf(jur_ptr, "\"jurisdiction\": \"%15[^\"]\"", jur_val) == 1) {
            strncpy(state->jurisdiction, jur_val, sizeof(state->jurisdiction)-1);
        }
    } else {
        strncpy(state->jurisdiction, "IEC62304", sizeof(state->jurisdiction)-1);
    }

    char* seq_ptr = strstr(buffer, "\"audit_sequence\":");
    if (seq_ptr) {
        unsigned long seq = 0;
        if (sscanf(seq_ptr, "\"audit_sequence\": %lu", &seq) == 1 || sscanf(seq_ptr, "\"audit_sequence\":%lu", &seq) == 1) {
            state->audit_sequence = (uint64_t)seq;
        } else {
            state->audit_sequence = 1000;
        }
    } else {
        state->audit_sequence = 1000;
    }

    return 0;
}

int save_governance_state(const GovernanceState* state, const char* filepath) {
    if (state == NULL || filepath == NULL) {
        return -1;
    }

    FILE* fp = fopen(filepath, "w");
    if (!fp) {
        return -1;
    }

    fprintf(fp, "{\n");
    fprintf(fp, "  \"state_id\": %u,\n", state->state_id);
    fprintf(fp, "  \"execution_permitted\": %s,\n", state->execution_permitted ? "true" : "false");
    fprintf(fp, "  \"immutable_lock\": %s,\n", state->immutable_lock ? "true" : "false");
    fprintf(fp, "  \"jurisdiction\": \"%s\",\n", state->jurisdiction);
    fprintf(fp, "  \"audit_sequence\": %lu\n", (unsigned long)state->audit_sequence);
    fprintf(fp, "}\n");

    fclose(fp);
    return 0;
}
