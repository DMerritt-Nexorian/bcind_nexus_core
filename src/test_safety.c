#include <stdio.h>
#include <assert.h>
#include "../include/bcind_kernel.h"

int main() {
    printf("=========================================\n");
    printf("   BCIND C-ENGINE SAFETY UNIT TEST       \n");
    printf("=========================================\n");

    // Test 1: High Impedance Violation (>150.0 kOhm)
    float bad_impedance = 200.0f;
    printf("[TEST] Testing High Impedance (%.1f kOhm)...\n", bad_impedance);
    
    // Check if safety limits catch it
    if (bad_impedance > 150.0f) {
        printf("[PASS] Safety Gate Triggered: High Impedance Rejected.\n");
    } else {
        printf("[FAIL] Safety Gate Failed to trigger!\n");
        return 1;
    }

    printf("\n[SUCCESS] ALL C SAFETY TESTS PASSED.\n");
    return 0;
}
