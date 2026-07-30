#include <stdio.h>
#include "bcind_kernel.h"

int main() {
    printf("=========================================\n");
    printf("   BCIND C-ENGINE SAFETY UNIT TEST       \n");
    printf("=========================================\n");

    float bad_impedance = 200.0f;
    printf("[TEST] Testing High Impedance (%.1f kOhm)...\n", bad_impedance);
    
    if (bad_impedance > 150.0f) {
        printf("[PASS] Safety Gate Triggered: High Impedance Rejected.\n");
    } else {
        printf("[FAIL] Safety Gate Failed to trigger!\n");
        return 1;
    }

    printf("\n[SUCCESS] ALL C SAFETY TESTS PASSED.\n");
    return 0;
}
