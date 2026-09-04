//! Write-Ahead Log (WAL) for durability

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// WAL entry type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WALEntry {
    Begin { tx_id: u64 },
    Write { tx_id: u64, page_id: u64, data: Vec<u8> },
    Commit { tx_id: u64 },
    Abort { tx_id: u64 },
}

/// Write-Ahead Log
pub struct WriteAheadLog {
    entries: Vec<WALEntry>,
}

impl WriteAheadLog {
    pub fn new() -> Self {
        WriteAheadLog {
            entries: Vec::new(),
        }
    }

    /// Append an entry to the log
    pub fn append(&mut self, entry: WALEntry) -> Result<()> {
        self.entries.push(entry);
        // In production: flush to disk
        Ok(())
    }

    /// Get all entries
    pub fn entries(&self) -> &[WALEntry] {
        &self.entries
    }
}

impl Default for WriteAheadLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wal_append() -> Result<()> {
        let mut wal = WriteAheadLog::new();
        wal.append(WALEntry::Begin { tx_id: 1 })?;
        wal.append(WALEntry::Commit { tx_id: 1 })?;

        assert_eq!(wal.entries().len(), 2);
        Ok(())
    }
}
