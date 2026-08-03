pub fn execute_reflex_action(reason: &str) {
    eprintln!("\n========================================================");
    eprintln!("   CRITICAL FAIL-SAFE REFLEX ACTION TRIGGERED");
    eprintln!(
        "   Reason: {}",
        if reason.is_empty() {
            "Unspecified System Boundary Fault"
        } else {
            reason
        }
    );
    eprintln!("   Status: Pipeline Latched / Output Terminated");
    eprintln!("========================================================\n");
}
