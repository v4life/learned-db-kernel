//! Statistics collection and maintenance
//!
//! Tracks table and column statistics for optimization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Table statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStats {
    pub table_id: u32,
    pub num_rows: usize,
    pub num_pages: usize,
    pub columns: HashMap<u32, ColumnStatistics>,
}

/// Column statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnStatistics {
    pub column_id: u32,
    pub num_distinct: usize,
    pub min_value: f64,
    pub max_value: f64,
    pub null_count: usize,
    pub average_width: usize,
}

/// Statistics collector
pub struct StatisticsCollector {
    pub table_stats: HashMap<u32, TableStats>,
}

impl StatisticsCollector {
    pub fn new() -> Self {
        StatisticsCollector {
            table_stats: HashMap::new(),
        }
    }

    /// Register a table
    pub fn register_table(&mut self, table_id: u32, num_rows: usize, num_pages: usize) {
        self.table_stats.insert(
            table_id,
            TableStats {
                table_id,
                num_rows,
                num_pages,
                columns: HashMap::new(),
            },
        );
    }

    /// Add column statistics
    pub fn add_column_stats(&mut self, table_id: u32, column_stats: ColumnStatistics) {
        if let Some(table_stats) = self.table_stats.get_mut(&table_id) {
            table_stats
                .columns
                .insert(column_stats.column_id, column_stats);
        }
    }

    /// Get table statistics
    pub fn get_table_stats(&self, table_id: u32) -> Option<&TableStats> {
        self.table_stats.get(&table_id)
    }
}

impl Default for StatisticsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_collector() {
        let mut collector = StatisticsCollector::new();
        collector.register_table(1, 1000, 10);

        let stats = collector.get_table_stats(1);
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().num_rows, 1000);
    }
}
