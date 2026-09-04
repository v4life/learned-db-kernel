//! Query Execution Layer
//!
//! Vectorized execution engine with transaction support.

pub mod catalog;
pub mod operators;
pub mod executor;
pub mod transaction;
pub mod wal;
pub mod recovery;

pub use catalog::Catalog;
pub use executor::QueryExecutor;
pub use transaction::TransactionManager;
pub use wal::WriteAheadLog;
