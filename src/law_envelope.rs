pub const CHANNELS: usize = 32;
pub const MAX_CONTACT_IMPEDANCE_KOHM: f64 = 150.0;

pub struct TelemetryFrame {
    pub hardware_contact_valid: bool,
    pub channel_impedance_kohm: [f64; CHANNELS],
    pub raw_signal_uv: [f64; CHANNELS],
    pub noise_floor_uv: [f64; CHANNELS],
    pub current_snr_db: f64,
}

impl Default for TelemetryFrame {
    fn default() -> Self {
        Self {
            hardware_contact_valid: true,
            channel_impedance_kohm: [45.0; CHANNELS],
            raw_signal_uv: [12.5; CHANNELS],
            noise_floor_uv: [25.0; CHANNELS],
            current_snr_db: 0.0,
        }
    }
}

pub fn check_law_envelope(frame: &TelemetryFrame, jurisdiction: &str) -> bool {
    if !frame.hardware_contact_valid {
        eprintln!("[LAW_ENVELOPE] Violation: Hardware contact flag reported invalid.");
        return false;
    }

    for i in 0..CHANNELS {
        if frame.channel_impedance_kohm[i] > MAX_CONTACT_IMPEDANCE_KOHM {
            eprintln!(
                "[LAW_ENVELOPE] Violation: Channel {} impedance ({:.2} kOhm) exceeds max limit ({:.2} kOhm).",
                i, frame.channel_impedance_kohm[i], MAX_CONTACT_IMPEDANCE_KOHM
            );
            return false;
        }
        if frame.channel_impedance_kohm[i] <= 0.0 {
            eprintln!(
                "[LAW_ENVELOPE] Violation: Channel {} impedance ({:.2} kOhm) is non-physical.",
                i, frame.channel_impedance_kohm[i]
            );
            return false;
        }
    }

    match jurisdiction {
        "ISO13485" | "IEC62304" | "DIN3105" => true,
        _ => {
            eprintln!(
                "[LAW_ENVELOPE] Warning: Unknown jurisdiction '{}'. Defaulting to strict fail-closed policy.",
                jurisdiction
            );
            false
        }
    }
}
