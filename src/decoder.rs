//! Simple spectral-feature decoder.
//! Converts EEG band powers into a continuous control signal and discrete direction.
//! Intentionally deterministic and free of online learning for the high-assurance core.

use crate::dsp::EegBandsPower;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Neutral,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct ControlOutput {
    pub continuous: f64, // normalized -1.0 .. 1.0
    pub direction: Direction,
    pub confidence: f64,
}

/// Decode band powers into a control command.
/// Uses alpha/beta ratio (classic motor-imagery style) plus theta for stability gating.
pub fn decode_band_powers(bands: &EegBandsPower) -> ControlOutput {
    let total = bands.delta + bands.theta + bands.alpha + bands.beta + bands.gamma + 1e-12;
    let alpha_rel = bands.alpha / total;
    let beta_rel = bands.beta / total;
    let theta_rel = bands.theta / total;

    // Simple ratio: higher beta relative to alpha → positive (e.g. "right" / engagement)
    let ratio = (beta_rel - alpha_rel) / (alpha_rel + beta_rel + 1e-6);
    let continuous = ratio.clamp(-1.0, 1.0);

    // Confidence drops if theta is dominant (often drowsiness / artifact)
    let confidence = (1.0 - theta_rel * 2.0).clamp(0.0, 1.0);

    let direction = if confidence < 0.3 {
        Direction::Neutral
    } else if continuous > 0.25 {
        Direction::Right
    } else if continuous < -0.25 {
        Direction::Left
    } else if beta_rel > 0.35 {
        Direction::Up
    } else {
        Direction::Neutral
    };

    ControlOutput {
        continuous,
        direction,
        confidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_alpha_dominant() {
        let bands = EegBandsPower {
            delta: 1.0,
            theta: 1.0,
            alpha: 10.0,
            beta: 2.0,
            gamma: 0.5,
        };
        let out = decode_band_powers(&bands);
        assert!(out.continuous < 0.0);
        assert!(matches!(
            out.direction,
            Direction::Left | Direction::Neutral
        ));
    }

    #[test]
    fn test_decode_beta_dominant() {
        let bands = EegBandsPower {
            delta: 1.0,
            theta: 1.0,
            alpha: 2.0,
            beta: 12.0,
            gamma: 1.0,
        };
        let out = decode_band_powers(&bands);
        assert!(out.continuous > 0.0);
    }
}
