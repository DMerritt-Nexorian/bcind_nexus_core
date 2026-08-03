# BCIND Nexus-Core Kernel

**Non-Invasive Brain-Computer Interface Neural Decoder (BCIND)**  
*Hardware-Bound Deterministic Software Governance Engine & Biophysical Signal Processor*

---

## 1. System Overview

The **BCIND Nexus-Core Kernel** is a high-reliability runtime engine designed for high-density, 32-channel non-invasive neural decoding. Implemented in pure, memory-safe Rust, it provides a deterministic validation layer that monitors electrode-scalp contact impedance, evaluates real-time Signal-to-Noise Ratio (SNR) thresholds, and enforces international medical and hardware safety parameters (IEC 62304 Class C, ISO 13485) before allowing downstream command execution.

---

## 2. Architecture & Compliance Invariants

* **32-Channel Biophysical Signal Processing:** Enforces a $150.0\text{ k}\Omega$ maximum contact impedance threshold and a $-10.0\text{ dB}$ minimum SNR admissibility threshold.
* **Contextual Enforcement & Admissibility Logic (CEAL):** A deterministic, formal state machine preventing unauthorized or unverified state transitions.
* **Fail-Safe Reflex Containment:** Immediate stream latching and output termination upon any boundary violation or hardware contact fault.
* **Model-Based Design Port:** Re-engineered from legacy MATLAB prototypes into pure Rust for high-performance, predictable, and memory-safe compilation.
* **Structured Auditing:** Generates tamper-evident, append-only structured JSON telemetry and state logging conforming to IEC 62304.

---

## 3. Directory Structure

```text
bcind_nexus_core/
├── Cargo.toml                    # Rust package manifest & dependencies
├── README.md                     # System technical documentation
├── AUDIT_REPORT.md               # Regulatory safety and hazard analysis
├── COMPLIANCE.md                 # Physical safety boundaries & compliance invariants
├── CONTRIBUTING.md               # Developer style guide, lint policies & PR gates
├── TECHNICAL_SPEC.md             # Signal processing DSP mathematical theory
├── OPENBCI_INTEGRATION.md        # Hardware interfacing guide via LSL & Serial
├── config/
│   └── governance_state.json     # Persistent governance state tracking
├── src/
│   ├── main.rs                   # Entry point and CLI argument parser
│   ├── lib.rs                    # Module declarations
│   ├── law_envelope.rs           # Channel impedance and jurisdiction guardrails
│   ├── admissibility.rs          # Real-time RMS SNR gate validation
│   ├── ceal.rs                   # Policy and state transition enforcer
│   ├── governance.rs             # JSON configuration loader & state preservation
│   ├── audit.rs                  # Structured JSON audit log formatter
│   ├── dsp.rs                    # High-performance filters, CAR, & EEG spectral analysis
│   └── immutable_core.rs         # Memory safety self-verification module
└── tests/
    └── mock_eeg_stream.rs        # Standalone verification signal generator and tests
```

---

## 4. Build and Verification Instructions

This repository is packed to allow building and testing with standard Rust toolchain commands.

### Dependencies
- Rust 1.70 or higher is required.

### Build the Core Kernel
```bash
cargo build --release
```

### Run the Signal Verification Suite
```bash
cargo test
```

### Run the Kernel CLI
```bash
cargo run -- --impedance 45.0 --signal 12.5 --noise 25.0
```
