//! Vectorized mathematical operations

/// Vector operations
pub struct VectorOps;

impl VectorOps {
    /// Element-wise addition
    pub fn add(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b).map(|(x, y)| x + y).collect()
    }

    /// Element-wise multiplication
    pub fn multiply(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b).map(|(x, y)| x * y).collect()
    }

    /// Dot product
    pub fn dot(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }

    /// Vector norm
    pub fn norm(v: &[f64]) -> f64 {
        v.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = VectorOps::add(&a, &b);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = VectorOps::dot(&a, &b);
        assert_eq!(result, 32.0);
    }
}
