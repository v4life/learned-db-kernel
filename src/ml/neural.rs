//! Simple neural network for cardinality estimation

use serde::{Deserialize, Serialize};

/// Activation functions
#[derive(Debug, Clone, Copy)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Tanh,
}

impl Activation {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            Activation::ReLU => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
        }
    }
}

/// Neural network layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dense {
    pub weights: Vec<Vec<f64>>,
    pub bias: Vec<f64>,
}

impl Dense {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let weights = vec![vec![0.1; input_size]; output_size];
        let bias = vec![0.0; output_size];
        Dense { weights, bias }
    }

    /// Forward pass
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        self.weights
            .iter()
            .zip(&self.bias)
            .map(|(w, b)| {
                let sum = w.iter().zip(input).map(|(wi, xi)| wi * xi).sum::<f64>() + b;
                Activation::ReLU.apply(sum)
            })
            .collect()
    }
}

/// Simple neural network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetwork {
    pub layers: Vec<Dense>,
}

impl NeuralNetwork {
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();
        for window in layer_sizes.windows(2) {
            layers.push(Dense::new(window[0], window[1]));
        }
        NeuralNetwork { layers }
    }

    /// Forward pass through network
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut output = input.to_vec();
        for layer in &self.layers {
            output = layer.forward(&output);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_network() {
        let nn = NeuralNetwork::new(&[2, 4, 1]);
        let input = vec![1.0, 2.0];
        let output = nn.forward(&input);
        assert_eq!(output.len(), 1);
    }
}
