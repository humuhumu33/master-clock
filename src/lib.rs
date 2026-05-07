//! # Prime Clock — Phase 1 anchored, UOR-grounded encoding
//!
//! Every constant, identity, and theorem in this crate derives from
//! exactly two irreducible parameters:
//!
//! - [`parameters::MODULUS_U`] = 30 (Gary's residue modulus, = 2·3·5)
//! - [`parameters::NUMERAL_BASE`] = 10 (decimal numeral system)
//!
//! Anything that cannot be derived from these two values via the
//! standard helpers in [`arithmetic`] (gcd, lcm, totient),
//! [`digital_root`], [`fibonacci`], or finite enumeration is not
//! present in this crate. The Master-Clock identity
//! `π(N) = D(N) − TF(N) + RF(N) + 3` and its calibration constants
//! χ, ζ, δ, k were audited and removed — they appear only in private
//! correspondence and do not arithmetically reproduce their own
//! tabulated values across five iterations of Gary's spec.
//!
//! What remains, all anchored:
//!
//! - **Phase 1 substrate.** [`u30`], [`u90`], [`r360`],
//!   [`appendix_a`], [`atlas_correspondence`], [`fibonacci`],
//!   [`units_z9`], [`digital_root`], [`dirichlet`].
//! - **Phase 1.5 — `(Z/9Z)*` substrate.** [`u90`] (24-element substrate
//!   coprime to 30 in `[1, 90]`) plus [`units_z9`] (the six units of
//!   `Z/9Z` and their 6×6 multiplication table).
//! - **Phase 2 — published Prime Spiral Sieve.** [`sieve`] and
//!   [`pi_function`]. Reproduces canonical `π(N)` exactly up to
//!   `10¹⁰` from `MODULUS_U = 30` alone.
//! - **Reference data only.** [`li`] holds the literature
//!   `π(N)` and `Li(N)` tables from OEIS A006880 / A057835. No
//!   derivation; just citation.
//! - **UOR substrate.** [`ring_substrate`] anchors th