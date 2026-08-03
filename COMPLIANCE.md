# Software Compliance Manual & Biophysical Invariants

This document establishes the formal compliance parameters, safety boundaries, and physical invariants enforced by `bcind_nexus_core`.

---

## 1. Regulatory Context

`bcind_nexus_core` is designed to serve as Software as a Medical Device (SaMD) under:
- **FDA Guidance for the Content of Premarket Submissions for Software Contained in Medical Devices** (classified as a "Major Level of Concern" when integrated with invasive and active non-invasive neural interface hardware).
- **ISO 13485:2016** (Quality Management System for Medical Devices).
- **IEC 62304:2006/AMD1:2015** (Medical device software — Software life cycle processes).

---

## 2. Biophysical Safety Envelope

To prevent dangerous current injection or thermal/electrical tissue damage in active systems, and to ensure high physical validity in passive recording systems, the following boundary constraints are strictly enforced at runtime:

### A. Scalp Contact Impedance
- **Constraint:** $Z_{contact} \le 150.0\text{ k}\Omega$ across all 32 recording channels.
- **Physical Rationale:** High-impedance connections act as major antennas for electromagnetic interference (EMI) and powerline noise (50/60 Hz). At levels $> 150.0\text{ k}\Omega$, the signal quality degrade precludes accurate phase decoding, yielding random command classification output.
- **Enforcement:** Checked in `src/law_envelope.rs`. Non-physical contact (such as values $\le 0.0\text{ k}\Omega$) indicates amplifier sensor saturation or hardware short-circuiting, triggering instant shutdown.

### B. Input Signal Range (Voltage Limits)
- **Constraint:** Expected raw physical EEG signal bounds are $\pm 200.0\text{ }\mu\text{V}$ relative to reference electrode.
- **Physical Rationale:** Biophysical EEG amplitudes rarely exceed $100\text{ }\mu\text{V}$ on the scalp. Transient voltages larger than $500\text{ }\mu\text{V}$ denote muscle contraction (EMG), ocular sweep (EOG), electrode displacement, or static discharge.
- **Enforcement:** Addressed by spatial filters and temporal bandpass filters. Extreme artifacts are clipped using statistical soft-thresholding in `src/dsp.rs`.

---

## 3. Data Processing Invariants

1. **No Out-of-Sequence Logging:** The system governance state preserves `audit_sequence`. Every transition is accompanied by a persistent write to the audit trail log file, ensuring tamper-evident accountability.
2. **Deterministic Processing Flow:**
   $$\text{Ingress} \longrightarrow \text{Core Integrity Check} \longrightarrow \text{CEAL Enforce} \longrightarrow \text{Law Envelope Verification} \longrightarrow \text{Admissibility (SNR) Check} \longrightarrow \text{DSP Chain} \longrightarrow \text{Egress}$$
3. **Strict Policy Enforcement (CEAL):** If the governance configuration parameter `execution_permitted` is set to `false`, or if `immutable_lock` is broken, the core immediately rejects decoding frames to prevent unauthorized execution.
