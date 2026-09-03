//! Learned Database Kernel
//!
//! A production-grade database kernel with ML-driven indexing, query optimization,
//! and adaptive buffer management.

pub mod compute;
pub mod execution;
pub mod index;
pub mod optimizer;
pub mod storage;

pub use storage::{buffer_pool::LearnedBufferPool, page::PageId};

/// Database kernel version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
