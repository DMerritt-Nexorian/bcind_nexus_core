Here is a clean, production-grade README.md tailored specifically for the unified, single-tier codebase you just built.
Save this in your repository root: bcind_nexus_core/README.md.
BCIND Nexus-Core Kernel
Non-Invasive Brain-Computer Interface Neural Decoder (BCIND)
Hardware-Bound Deterministic Software Governance Engine & Biophysical Signal Processor
System Overview
The BCIND Nexus-Core Kernel is an institutional-grade runtime engine designed for high-density, 32-channel non-invasive neural decoding. It provides a hard-wired safety layer that validates electrode-scalp contact impedance, computes signal-to-noise ratio (SNR) thresholds in real time, and enforces international medical and hardware safety envelopes (ISO 13485, IEC 62304, DIN 3105) before permitting neural execution signals.
The architecture is fully deterministic, persistent, and unencumbered by artificial paywalls or feature toggles.
Key Features
 * 32-Channel Biophysical Signal Processing: Enforces 150.0 \text{ k}\Omega maximum contact impedance limits and a -10.0 \text{ dB} minimum SNR admissibility gate.
 * Contextual Enforcement & Admissibility Logic (CEAL): Hard-wired state machine preventing unverified or non-compliant neural state transitions.
 * Fail-Safe Reflex Containment: Instantaneous execution latching and telemetry termination upon boundary breach or memory fault.
 * Model-Based Design (MBD): Integrated MATLAB simulation environment for state-space neural dynamics and Monte Carlo validation.
 * Audit Persistence: Append-only structured JSON telemetry and state logging for full regulatory compliance.
Directory Structure
bcind_nexus_core/
├── Makefile                        # Multi-target build configuration
├── README.md                       # Architecture & setup manual
├── config/
│   └── governance_state.json       # Persistent governance & sequence state
├── include/
│   └── bcind_kernel.h              # Unified system header & structure definitions
├── matlab/
│   └── verify_bcind_mbd.m          # MATLAB state-space & Monte Carlo simulation
└── src/
    ├── admissibility_gate.c        # RMS SNR signal validation engine
    ├── audit.c                     # Structured JSON audit log writer
    ├── ceal.c                      # Contextual policy enforcement module
    ├── governance.c                # JSON configuration state loader/saver
    ├── immutable_core.c           # Memory integrity verification
    ├── law_envelope.c              # Regulatory and impedance safety guardrails
    ├── main.c                      # Entry point & CLI argument parser
    └── reflex.c                    # Critical fail-safe containment logic

Quick Start
Prerequisites
 * C Compiler: gcc or clang (C11 support required)
 * Build Tool: GNU make
 * Optional: MATLAB / GNU Octave (for MBD simulation verification)
Compilation
Clone the repository and compile using the provided Makefile:
# Build the production executable
make all

# Clean build artifacts
make clean

The compiled binary will be placed in bin/bcind_core.
Usage & Execution
Run the binary directly with default telemetry parameters:
./bin/bcind_core

Dynamic Telemetry Overrides
You can inject telemetry variables via command-line flags to test system boundary responses:
# Example: Run with custom contact impedance and microvolt signals
./bin/bcind_core --impedance 35.0 --signal 18.0 --noise 12.0 --jurisdiction IEC62304

# Example: Simulate a boundary violation (High Impedance)
./bin/bcind_core --impedance 200.0

CLI Command Reference
| Flag | Type | Default | Description |
|---|---|---|---|
| --impedance <val> | float | 45.0 | Set electrode contact impedance (\text{k}\Omega). Limit: 150.0 \text{ k}\Omega. |
| --signal <val> | float | 12.5 | Raw signal amplitude (\mu\text{V}). |
| --noise <val> | float | 25.0 | Noise floor amplitude (\mu\text{V}). |
| --jurisdiction <str> | string | IEC62304 | Regulatory framework (ISO13485, IEC62304, DIN3105). |
| --config <path> | string | config/... | Path to persistent governance state file. |
| --audit <path> | string | audit_export.json | Path to JSON audit log output. |
Model-Based Design (MBD) Verification
To validate system performance against state-space matrix dynamics and run a 100-cycle Monte Carlo verification sweep:
 * Open MATLAB or GNU Octave.
 * Navigate to the matlab/ directory.
 * Run the script:
   run('verify_bcind_mbd.m')

