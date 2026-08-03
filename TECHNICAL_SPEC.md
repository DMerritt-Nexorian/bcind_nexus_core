# TECHNICAL_SPEC.md: Biophysical Signal Processing Theory & Implementation

This specification outlines the mathematical formulations, filter coefficients, and algorithmic pipelines implemented within the `bcind_nexus_core` engine.

---

## 1. Architectural Pipeline

The system processes incoming high-density neural signals in discrete spatial-temporal steps:

```text
  [32-Channel EEG Ingress]
             │
             ▼
  [Common Average Reference]  <─── Spatial Filter (Noise cancellation)
             │
             ▼
  [Biquad Bandpass Filter]    <─── 2nd-Order Butterworth (0.5 - 45.0 Hz)
             │
             ▼
  [Biquad Notch Filter]       <─── 2nd-Order Notch (50.0 Hz or 60.0 Hz)
             │
             ▼
  [Statistical Outlier Clip]  <─── Artifact Suppressor
             │
             ▼
  [Fast Fourier Transform]    <─── Hanning Window -> FFT -> Power Integration
             │
             ▼
     [EEG Band Powers]        <─── Output: Delta, Theta, Alpha, Beta, Gamma
```

---

## 2. Mathematical Formulations

### A. Spatial Filtering: Common Average Reference (CAR)
To remove global common-mode electrical noise (such as line hum or movement artifacts present on the reference electrode), we compute:
$$y_i[n] = x_i[n] - \frac{1}{C}\sum_{j=1}^{C} x_j[n]$$
where $x_i[n]$ is the raw voltage of channel $i$ at sample $n$, and $C$ is the total channel count ($C = 32$).

### B. Temporal Filtering: Biquad Direct Form II Transposed
Real-time filtering is executed using Second-Order Sections (Biquads) represented by the transfer function:
$$H(z) = \frac{b_0 + b_1 z^{-1} + b_2 z^{-2}}{1 + a_1 z^{-1} + a_2 z^{-2}}$$
The difference equations are implemented as a Direct Form II Transposed state-space structure to minimize numerical quantization errors and floating-point noise:
$$y[n] = b_0 x[n] + s_1[n-1]$$
$$s_1[n] = b_1 x[n] - a_1 y[n] + s_2[n-1]$$
$$s_2[n] = b_2 x[n] - a_2 y[n]$$
where $s_1$ and $s_2$ are the delay registers preserving state between consecutive samples.

### C. Power Spectral Density (PSD)
To integrate power inside physiological bands, the segment of signal $w[n]$ is windowed with a Hanning window:
$$w[n] = 0.5 \left(1 - \cos\left(\frac{2\pi n}{N-1}\right)\right)$$
The windowed discrete Fourier transform (DFT) is calculated:
$$X[k] = \sum_{n=0}^{N-1} w[n] x[n] e^{-j \frac{2\pi}{N} k n}$$
The single-sided power spectral density (PSD) $P[k]$ for a sampling rate $f_s$ is:
$$P[k] = \frac{2 |X[k]|^2}{N \cdot f_s}, \quad \text{for } 1 \le k < \frac{N}{2}$$
$$P[k] = \frac{|X[k]|^2}{N \cdot f_s}, \quad \text{for } k = 0, \frac{N}{2}$$

Frequency band integration computes the total band power by integrating over the relevant frequency boundaries:
$$\text{Band Power} = \sum_{f = f_{min}}^{f_{max}} P(f) \cdot \Delta f$$
where $\Delta f = \frac{f_s}{N}$ is the frequency resolution.

- **Delta:** $1.0 - 4.0\text{ Hz}$
- **Theta:** $4.0 - 8.0\text{ Hz}$
- **Alpha:** $8.0 - 13.0\text{ Hz}$
- **Beta:** $13.0 - 30.0\text{ Hz}$
- **Gamma:** $30.0 - 100.0\text{ Hz}$
