use bcind_core::admissibility::evaluate_admissibility;
use bcind_core::audit::log_audit_event;
use bcind_core::ceal::ceal_enforce_policy;
use bcind_core::governance::{load_governance_state, save_governance_state};
use bcind_core::immutable_core::verify_immutable_core;
use bcind_core::law_envelope::check_law_envelope;
use bcind_core::law_envelope::{CHANNELS, TelemetryFrame};
use bcind_core::reflex::execute_reflex_action;
use std::env;
use std::process;

const SYSTEM_VERSION: &str = "0.2.0-rust";

fn print_usage(prog: &str) {
    println!("Usage: {} [OPTIONS]", prog);
    println!("Options:");
    println!(
        "  --impedance <val>   Set scalp contact impedance for all channels in kOhm (default: 45.0)"
    );
    println!("  --signal <val>      Set microvolt signal amplitude in uV (default: 12.5)");
    println!("  --noise <val>       Set microvolt noise amplitude in uV (default: 25.0)");
    println!(
        "  --jurisdiction <str>Set jurisdiction code [ISO13485|IEC62304|DIN3105] (default: IEC62304)"
    );
    println!(
        "  --config <path>     Path to governance JSON config (default: config/governance_state.json)"
    );
    println!("  --audit <path>      Path to audit log JSON output (default: audit_export.json)");
    println!("  --help              Display this help message");
}

fn main() {
    // Initialize structured logging with a default level of info if not overridden
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    let mut input_impedance = 45.0;
    let mut input_signal = 12.5;
    let mut input_noise = 25.0;
    let mut jurisdiction = "IEC62304".to_string();
    let mut config_path = "config/governance_state.json".to_string();
    let mut audit_path = "audit_export.json".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--impedance" => {
                if i + 1 < args.len() {
                    input_impedance = args[i + 1].parse().unwrap_or(45.0);
                    i += 2;
                } else {
                    eprintln!("Error: --impedance requires a value");
                    process::exit(1);
                }
            }
            "--signal" => {
                if i + 1 < args.len() {
                    input_signal = args[i + 1].parse().unwrap_or(12.5);
                    i += 2;
                } else {
                    eprintln!("Error: --signal requires a value");
                    process::exit(1);
                }
            }
            "--noise" => {
                if i + 1 < args.len() {
                    input_noise = args[i + 1].parse().unwrap_or(25.0);
                    i += 2;
                } else {
                    eprintln!("Error: --noise requires a value");
                    process::exit(1);
                }
            }
            "--jurisdiction" => {
                if i + 1 < args.len() {
                    jurisdiction = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --jurisdiction requires a value");
                    process::exit(1);
                }
            }
            "--config" => {
                if i + 1 < args.len() {
                    config_path = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --config requires a value");
                    process::exit(1);
                }
            }
            "--audit" => {
                if i + 1 < args.len() {
                    audit_path = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --audit requires a value");
                    process::exit(1);
                }
            }
            "--help" | "-h" => {
                print_usage(prog);
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                print_usage(prog);
                process::exit(1);
            }
        }
    }

    log::info!("========================================================");
    log::info!("   BCIND NEXUS-GENESIS KERNEL v{}", SYSTEM_VERSION);
    log::info!("   Non-Invasive Brain-Computer Interface Neural Decoder");
    log::info!("========================================================\n");

    if !verify_immutable_core() {
        execute_reflex_action("Immutable Core Integrity Check Failed");
        process::exit(1);
    }

    let mut gov_state = match load_governance_state(&config_path) {
        Ok(state) => state,
        Err(e) => {
            log::error!(
                "[ERROR] Failed to load or create governance state at {}: {}",
                config_path,
                e
            );
            process::exit(1);
        }
    };

    if !jurisdiction.is_empty() {
        gov_state.jurisdiction = jurisdiction;
    }

    let mut frame = TelemetryFrame {
        hardware_contact_valid: true,
        channel_impedance_kohm: [input_impedance; CHANNELS],
        raw_signal_uv: [input_signal; CHANNELS],
        noise_floor_uv: [input_noise; CHANNELS],
        current_snr_db: 0.0,
    };

    log::info!(
        "[INIT] System state loaded. Sequence: {} | Jurisdiction: {}",
        gov_state.audit_sequence,
        gov_state.jurisdiction
    );

    if !ceal_enforce_policy(&frame, &gov_state) {
        log_audit_event(
            &audit_path,
            "KERNEL_EXEC",
            "REJECTED_CEAL",
            0.0,
            gov_state.audit_sequence,
        );
        process::exit(1);
    }

    if !check_law_envelope(&frame, &gov_state.jurisdiction) {
        execute_reflex_action("Law Envelope Boundary Breach");
        log_audit_event(
            &audit_path,
            "KERNEL_EXEC",
            "REJECTED_LAW_ENVELOPE",
            0.0,
            gov_state.audit_sequence,
        );
        process::exit(1);
    }

    if !evaluate_admissibility(&mut frame) {
        execute_reflex_action("Admissibility Gate Failed: Insufficient SNR");
        log_audit_event(
            &audit_path,
            "KERNEL_EXEC",
            "REJECTED_ADMISSIBILITY",
            frame.current_snr_db,
            gov_state.audit_sequence,
        );
        process::exit(1);
    }

    log::info!("[SUCCESS] Telemetry frame admissible!");
    log::info!(
        "          Calculated SNR: {:.2} dB | Contact Impedance: {:.2} kOhm",
        frame.current_snr_db,
        input_impedance
    );

    gov_state.audit_sequence += 1;
    if let Err(e) = save_governance_state(&gov_state, &config_path) {
        log::error!("[ERROR] Failed to save governance state: {}", e);
    } else {
        log::info!(
            "[STATE] Persisted sequence {} to {}",
            gov_state.audit_sequence,
            config_path
        );
    }

    log_audit_event(
        &audit_path,
        "KERNEL_EXEC",
        "PASS",
        frame.current_snr_db,
        gov_state.audit_sequence,
    );
}
