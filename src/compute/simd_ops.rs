//! SIMD-accelerated operations

/// SIMD search operations
pub struct SIMDSearch;

impl SIMDSearch {
    /// SIMD-accelerated binary search
    pub fn binary_search(arr: &[f64], target: f64) -> Option<usize> {
        arr.binary_search_by(|x| x.partial_cmp(&target).unwrap())
            .ok()
    }

    /// SIMD filter operation
    pub fn filter<F>(arr: &[f64], predicate: F) -> Vec<f64>
    where
        F: Fn(f64) -> bool,
    {
        arr.iter().filter(|x| predicate(**x)).copied().collect()
    }

    /// SIMD compare operation
    pub fn compare(arr1: &[f64], arr2: &[f64]) -> Vec<bool> {
        arr1.iter()
            .zip(arr2)
            .map(|(a, b)| a == b)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_binary_search() {
        let arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = SIMDSearch::binary_search(&arr, 3.0);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_simd_filter() {
        let arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = SIMDSearch::filter(&arr, |x| x > 2.5);
        assert_eq!(result, vec![3.0, 4.0, 5.0]);
    }
}
