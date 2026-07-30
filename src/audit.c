#include "../include/bcind_kernel.h"

void log_audit_event(const char* filepath, const char* action, const char* status, double snr_db, uint64_t seq) {
    if (filepath == NULL) return;

    FILE* fp = fopen(filepath, "a");
    if (!fp) return;

    time_t now = time(NULL);
    char time_buf[64];
    struct tm* tm_info = gmtime(&now);
    strftime(time_buf, sizeof(time_buf), "%Y-%m-%dT%H:%M:%SZ", tm_info);

    fprintf(fp, "{\"timestamp\":\"%s\",\"sequence\":%lu,\"action\":\"%s\",\"status\":\"%s\",\"snr_db\":%.2f}\n",
            time_buf, (unsigned long)seq, action ? action : "UNKNOWN", status ? status : "UNKNOWN", snr_db);

    fclose(fp);
    printf("[AUDIT] Event sequence %lu appended to %s\n", (unsigned long)seq, filepath);
}
