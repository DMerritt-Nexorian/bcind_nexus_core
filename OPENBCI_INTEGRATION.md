# OpenBCI Integration and LSL Interface Guide

This guide describes how to interface `bcind_nexus_core` with OpenBCI Cyton, Ganglion, and Galea hardware systems in Berlin or multi-site research labs.

---

## 1. Hardware Stream Architecture

```text
  [OpenBCI Cyton/Ganglion/Galea]
                 │
                 ▼ (USB Dongle / Bluetooth / WiFi)
         [OpenBCI GUI / BrainFlow]
                 │
                 ▼ (Lab Streaming Layer - LSL)
         [LSL EEG Outlet Stream]
                 │
                 ▼ (Network / Sockets)
   [bcind_nexus_core LSL Ingress]
```

To achieve ultra-reliable, memory-safe streaming, `bcind_nexus_core` processes data originating from standard OpenBCI configurations:
- **Cyton (8 Channels / 16 Channels with Daisy):** Sample rate $250\text{ Hz}$.
- **Ganglion (4 Channels):** Sample rate $200\text{ Hz}$.
- **Galea (16 Channels EEG + EMG + EDA + PPG):** Sample rate $250\text{ Hz}$ / $500\text{ Hz}$.

---

## 2. Setting Up Lab Streaming Layer (LSL)

Lab Streaming Layer (LSL) is the standard transport protocol for high-density biophysical signals. It provides sub-millisecond clock synchronization and jitter correction across distributed networks.

### Interfacing Steps
1. **Connect Hardware to OpenBCI GUI:**
   - Plug in your Cyton dongle or turn on your Ganglion.
   - Start the OpenBCI GUI. Select your connection type and start the hardware stream.
2. **Enable LSL Broadcast:**
   - In the OpenBCI GUI, go to the **Networking** widget.
   - Select **LSL** as the protocol type.
   - Choose the data stream type: **EEG**. Set the stream name (e.g., `openbci_eeg`).
   - Click **Start Stream**. The GUI will broadcast EEG samples over the local network.
3. **Consume Stream with `bcind_nexus_core`:**
   - The core reads the incoming telemetry packet stream.
   - For Rust-based LSL consumers, integrate standard `liblsl` or use socket connections to ingest frames into the `TelemetryFrame` structures.
   - Pass frames through the real-time safety pipeline:
     ```rust
     use bcind_core::law_envelope::{TelemetryFrame, check_law_envelope};
     use bcind_core::admissibility::evaluate_admissibility;

     fn on_lsl_frame_received(data: &[f64; 32], impedances: &[f64; 32]) {
         let mut frame = TelemetryFrame {
             hardware_contact_valid: true,
             channel_impedance_kohm: *impedances,
             raw_signal_uv: *data,
             noise_floor_uv: [2.5; 32], // baseline estimation
             current_snr_db: 0.0,
         };

         if check_law_envelope(&frame, "IEC62304") && evaluate_admissibility(&mut frame) {
             // Pass to DSP and classification pipeline
             println!("Frame admissible. SNR: {:.2} dB", frame.current_snr_db);
         } else {
             // Trigger fail-safe reflexes
             eprintln!("Compliance or SNR boundary breach detected!");
         }
     }
     ```

---

## 3. Serial Port Interface Fallback

If LSL is not available, direct serial connections can be established with the Cyton board using standard serial command packets (e.g. at `115200` baud rate).
1. Configure serial port permissions on Linux:
   ```bash
   sudo usermod -a -G dialout $USER
   ```
2. Parse binary Cyton packets (33 bytes) consisting of:
   - Start Byte: `0xA0` (1 byte)
   - Sample Number: (1 byte)
   - EEG Channel Data: 24-bit signed integers (24 bytes for 8 channels)
   - Aux Data: (6 bytes)
   - Stop Byte: `0xC0` (1 byte)
