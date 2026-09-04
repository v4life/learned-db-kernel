//! Machine learning models for index prediction

use serde::{Deserialize, Serialize};

/// Simple linear regression model: y = m*x + b
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearModel {
    pub m: f64, // slope
    pub b: f64, // intercept
}

impl LinearModel {
    pub fn new(m: f64, b: f64) -> Self {
        LinearModel { m, b }
    }

    /// Predict output given input
    pub fn predict(&self, x: f64) -> f64 {
        self.m * x + self.b
    }

    /// Fit model using simple OLS regression
    pub fn fit(x_values: &[f64], y_values: &[f64]) -> Option<Self> {
        if x_values.len() < 2 || x_values.len() != y_values.len() {
            return None;
        }

        let n = x_values.len() as f64;
        let mean_x = x_values.iter().sum::<f64>() / n;
        let mean_y = y_values.iter().sum::<f64>() / n;

        let numerator = x_values
            .iter()
            .zip(y_values.iter())
            .map(|(x, y)| (x - mean_x) * (y - mean_y))
            .sum::<f64>();

        let denominator = x_values
            .iter()
            .map(|x| (x - mean_x).powi(2))
            .sum::<f64>();

        if denominator.abs() < 1e-10 {
            return None;
        }

        let m = numerator / denominator;
        let b = mean_y - m * mean_x;

        Some(LinearModel { m, b })
    }
}

/// Piecewise linear model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiecewiseLinearModel {
    pub segments: Vec<LinearModel>,
    pub boundaries: Vec<f64>,
}

impl PiecewiseLinearModel {
    pub fn new(segments: Vec<LinearModel>, boundaries: Vec<f64>) -> Self {
        PiecewiseLinearModel { segments, boundaries }
    }

    /// Predict using appropriate segment
    pub fn predict(&self, x: f64) -> f64 {
        for (i, &boundary) in self.boundaries.iter().enumerate() {
            if x <= boundary && i < self.segments.len() {
                return self.segments[i].predict(x);
            }
        }
        self.segments
            .last()
            .map(|m| m.predict(x))
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_model_fit() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        let model = LinearModel::fit(&x, &y).expect("Fit failed");
        assert!((model.m - 2.0).abs() < 0.01);
        assert!((model.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_linear_model_predict() {
        let model = LinearModel::new(2.0, 1.0);
        assert!((model.predict(2.0) - 5.0).abs() < 0.01);
    }
}
