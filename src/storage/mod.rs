//! Storage Layer
//!
//! Low-level storage management including page handling, disk I/O, and buffer pool.

pub mod buffer_pool;
pub mod disk_manager;
pub mod page;

pub use buffer_pool::LearnedBufferPool;
pub use disk_manager::DiskManager;
pub use page::{Page, PageId, SlottedPage};
