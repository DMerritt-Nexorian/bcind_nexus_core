#include "../include/bcind_kernel.h"

void execute_reflex_action(const char* reason) {
    fprintf(stderr, "\n========================================================\n");
    fprintf(stderr, "   CRITICAL FAIL-SAFE REFLEX ACTION TRIGGERED\n");
    fprintf(stderr, "   Reason: %s\n", reason ? reason : "Unspecified System Boundary Fault");
    fprintf(stderr, "   Status: Pipeline Latched / Output Terminated\n");
    fprintf(stderr, "========================================================\n\n");
}
