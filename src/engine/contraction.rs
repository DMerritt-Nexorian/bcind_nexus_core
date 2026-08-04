/// Wrapper for the state vector to safely implement Default for arbitrary dimensions.
#[derive(Copy, Clone, Debug)]
pub struct LatentState<const DIM: usize>(pub [f32; DIM]);

impl<const DIM: usize> Default for LatentState<DIM> {
    fn default() -> Self {
        Self([0.0f32; DIM])
    }
}

/// Mathematical Invariants Enforcement Engine
pub struct ContractionEngine<const DIM: usize> {
    pub contraction_rate: f32, // c > 0
    pub max_radius: f32,       // Bound for Convex Set C
}

impl<const DIM: usize> ContractionEngine<DIM> {
    pub fn new(contraction_rate: f32, max_radius: f32) -> Self {
        assert!(contraction_rate > 0.0, "Contraction rate c must be > 0");
        Self {
            contraction_rate,
            max_radius,
        }
    }

    /// Evaluates contractive rate constraint: d/dt ||dx(t)|| <= -c ||dx(t)||
    #[inline(always)]
    pub fn verify_contractive_rate(&self, dx_current: &[f32; DIM], d_dx_dt: &[f32; DIM]) -> bool {
        let norm_dx = self.l2_norm(dx_current);
        let norm_d_dx = self.l2_norm(d_dx_dt);

        // Ensures rate of change does not exceed safe contraction bound
        norm_d_dx <= -self.contraction_rate * norm_dx
    }

    /// Convex Projection Operator: Pi_C(W_t - eta * grad_L)
    /// Projects parameters onto bounded set C to prevent adversarial drift.
    #[inline(always)]
    #[allow(clippy::needless_range_loop)]
    pub fn project_weights(&self, weights: &mut [f32; DIM], grad: &[f32; DIM], eta: f32) {
        // Step 1: SGD Step
        for i in 0..DIM {
            weights[i] -= eta * grad[i];
        }

        // Step 2: Projection onto Convex Set C (Hyper-Sphere Bound)
        let current_norm = self.l2_norm(weights);
        if current_norm > self.max_radius {
            let scale = self.max_radius / current_norm;
            for i in 0..DIM {
                weights[i] *= scale; // Scale back into set C
            }
        }
    }

    #[inline(always)]
    fn l2_norm(&self, vec: &[f32; DIM]) -> f32 {
        let mut sum = 0.0f32;
        for &val in vec.iter() {
            sum += val * val;
        }
        sum.sqrt()
    }
}
