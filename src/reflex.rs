pub fn execute_reflex_action(reason: &str) {
    log::error!("\n========================================================");
    log::error!("   CRITICAL FAIL-SAFE REFLEX ACTION TRIGGERED");
    log::error!(
        "   Reason: {}",
        if reason.is_empty() {
            "Unspecified System Boundary Fault"
        } else {
            reason
        }
    );
    log::error!("   Status: Pipeline Latched / Output Terminated");
    log::error!("========================================================\n");
}
