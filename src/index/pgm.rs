//! Piecewise Geometric Model (PGM) Index
//!
//! Partitions sorted keys into segments, each covered by a linear model.

use crate::index::models::LinearModel;
use serde::{Deserialize, Serialize};

/// Segment in a PGM index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PGMSegment {
    pub model: LinearModel,
    pub start_key: f64,
    pub end_key: f64,
    pub start_pos: usize,
    pub end_pos: usize,
}

impl PGMSegment {
    pub fn new(
        model: LinearModel,
        start_key: f64,
        end_key: f64,
        start_pos: usize,
        end_pos: usize,
    ) -> Self {
        PGMSegment {
            model,
            start_key,
            end_key,
            start_pos,
            end_pos,
        }
    }

    pub fn predict_position(&self, key: f64) -> usize {
        let predicted = self.model.predict(key) as usize;
        predicted.clamp(self.start_pos, self.end_pos)
    }
}

/// Piecewise Geometric Model Index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PGMIndex {
    pub segments: Vec<PGMSegment>,
    pub keys: Vec<f64>,
    pub error_bound: usize,
}

impl PGMIndex {
    pub fn new(segments: Vec<PGMSegment>, keys: Vec<f64>, error_bound: usize) -> Self {
        PGMIndex {
            segments,
            keys,
            error_bound,
        }
    }

    /// Build PGM index from sorted keys
    pub fn build(keys: Vec<f64>, error_bound: usize) -> Self {
        let mut segments = Vec::new();

        let positions: Vec<usize> = (0..keys.len()).collect();
        let mut i = 0;

        while i < keys.len() {
            let start_pos = i;
            let start_key = keys[i];
            let mut j = (i + 1).min(keys.len());

            // Find segment end with bounded error
            while j < keys.len() {
                let segment_keys = &keys[i..=j];
                let segment_positions: Vec<f64> = (0..=j - i).map(|x| x as f64).collect();

                if let Some(model) = LinearModel::fit(
                    segment_keys,
                    &segment_positions
                        .iter()
                        .map(|x| *x + start_pos as f64)
                        .collect::<Vec<_>>(),
                ) {
                    // Check error
                    let mut max_error = 0usize;
                    for (k, pos) in segment_keys.iter().zip(&segment_positions) {
                        let predicted = model.predict(*k) as usize;
                        let actual = (*pos as usize) + start_pos;
                        let error = (predicted as i32 - actual as i32).abs() as usize;
                        max_error = max_error.max(error);
                    }

                    if max_error <= error_bound {
                        j += 1;
                        continue;
                    }
                }
                break;
            }

            let end_pos = j - 1;
            let end_key = keys[end_pos];

            if let Some(model) = LinearModel::fit(
                &keys[i..=end_pos],
                &(0..=end_pos - i)
                    .map(|x| (x + start_pos) as f64)
                    .collect::<Vec<_>>(),
            ) {
                let segment = PGMSegment::new(model, start_key, end_key, start_pos, end_pos);
                segments.push(segment);
            }

            i = end_pos + 1;
        }

        PGMIndex {
            segments,
            keys,
            error_bound,
        }
    }

    /// Search using PGM segments
    pub fn search(&self, key: f64) -> Option<usize> {
        // Find appropriate segment
        let segment = self
            .segments
            .iter()
            .find(|s| key >= s.start_key && key <= s.end_key)?;

        let predicted_pos = segment.predict_position(key);
        let start = predicted_pos.saturating_sub(self.error_bound);
        let end = (predicted_pos + self.error_bound).min(self.keys.len());

        self.keys[start..end]
            .binary_search_by(|k| k.partial_cmp(&key).unwrap())
            .ok()
            .map(|offset| start + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgm_build() {
        let keys = vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 20.0];
        let pgm = PGMIndex::build(keys, 1);
        assert!(!pgm.segments.is_empty());
    }

    #[test]
    fn test_pgm_search() {
        let keys = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let pgm = PGMIndex::build(keys, 1);
        let result = pgm.search(3.0);
        assert!(result.is_some());
    }
}
