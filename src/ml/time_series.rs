//! Time series prediction for workload forecasting

use serde::{Deserialize, Serialize};

/// Simple autoregressive model for time series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARModel {
    pub coefficients: Vec<f64>,
    pub history: Vec<f64>,
}

impl ARModel {
    pub fn new(order: usize) -> Self {
        ARModel {
            coefficients: vec![0.1; order],
            history: Vec::new(),
        }
    }

    /// Add observation
    pub fn observe(&mut self, value: f64) {
        self.history.push(value);
    }

    /// Predict next value
    pub fn predict(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }

        let start = self.history.len().saturating_sub(self.coefficients.len());
        self.history[start..]
            .iter()
            .zip(&self.coefficients)
            .map(|(h, c)| h * c)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ar_model() {
        let mut model = ARModel::new(2);
        model.observe(1.0);
        model.observe(2.0);
        let pred = model.predict();
        assert!(pred > 0.0);
    }
}
