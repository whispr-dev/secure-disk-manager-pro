use crate::error::Result;
use std::fs;
use std::path::Path;

/// C++ equivalent: `SIMD_Entropy::calculateEntropy`.
pub fn calculate_entropy_bytes(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut freq = [0_u64; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }
    let total = data.len() as f64;
    freq.iter().filter(|&&count| count > 0).fold(0.0, |acc, &count| {
        let p = count as f64 / total;
        acc - p * p.log2()
    })
}

pub fn calculate_entropy(content: &str) -> f64 {
    calculate_entropy_bytes(content.as_bytes())
}

/// C++ equivalent: `SIMD_Entropy::calculateFileEntropy`.
pub fn calculate_file_entropy(path: impl AsRef<Path>) -> Result<f64> {
    Ok(calculate_entropy_bytes(&fs::read(path)?))
}
