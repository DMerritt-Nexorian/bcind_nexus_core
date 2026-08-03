use chrono::Utc;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
struct AuditEvent<'a> {
    timestamp: String,
    sequence: u64,
    action: &'a str,
    status: &'a str,
    snr_db: f64,
}

pub fn log_audit_event<P: AsRef<Path>>(
    filepath: P,
    action: &str,
    status: &str,
    snr_db: f64,
    seq: u64,
) {
    let timestamp = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let event = AuditEvent {
        timestamp,
        sequence: seq,
        action,
        status,
        snr_db,
    };

    if let Ok(serialized) = serde_json::to_string(&event) {
        if let Some(parent) = filepath.as_ref().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&filepath) {
            let _ = writeln!(file, "{}", serialized);
            println!(
                "[AUDIT] Event sequence {} appended to {}",
                seq,
                filepath.as_ref().display()
            );
        }
    }
}
