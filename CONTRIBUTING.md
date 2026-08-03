# Engineering Contribution & Software Governance Guidelines

All development and integration contributions to `bcind_nexus_core` must strictly adhere to the following professional engineering policies.

---

## 1. Coding Standards & Safety Constraints

1. **Zero Raw Pointer Allocations:**
   - No `unsafe` blocks are permitted in core DSP or policy-enforcement code unless wrapping an external, audited C-FFI header.
   - All buffers must use standard, bounds-checked Rust containers (`Vec`, standard arrays, or slices).
2. **Explicit Type Signatures:**
   - All public functions must have explicit, statically typed parameter and return types.
   - Do not use dynamically typed values or unchecked type coercions.
3. **No Unhandled Panics:**
   - Any function that can fail under physical operating conditions (such as loading config files, writing logs, parsing inputs) must return a `Result<T, E>`.
   - The use of `unwrap()` or `expect()` in library code is strictly forbidden. Always handle errors gracefully with matching, fallback, or propagate syntax (`?`).

---

## 2. Static Analysis & Lint Rules

Before submitting any code changes, developer modules must pass:
- **Rust Clippy:** Run `cargo clippy --all-targets -- -D warnings`. Code must have zero warnings.
- **Rustfmt:** Run `cargo fmt --all -- --check` to enforce deterministic, uniform formatting.
- **Unit and Integration Test Suites:** All tests must pass with a 100% success rate on local machines and target CI environments before merge.

---

## 3. Pull Request & Branch Protection Policies

1. **No Direct Merges to `main`:**
   - The `main` branch is protected. All modifications must be submitted via feature branches and Pull Requests.
2. **Deterministic Commit Messages:**
   - Commit messages must follow the standard Conventional Commits specification:
     - `feat(core): ...`
     - `fix(dsp): ...`
     - `refactor(gov): ...`
     - `docs(compliance): ...`
     - `ci(workflow): ...`
3. **CI Pipeline Validation:**
   - Every Pull Request automatically triggers the `.github/workflows/ci.yml` and `.github/workflows/audit_verification.yml` suites. Any step failure blocks the Pull Request from merging.
