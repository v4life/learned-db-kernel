//! Computation Utilities
//!
//! SIMD and mathematical acceleration primitives.

pub mod simd_ops;
pub mod vector_math;

pub use simd_ops::SIMDSearch;
pub use vector_math::VectorOps;
