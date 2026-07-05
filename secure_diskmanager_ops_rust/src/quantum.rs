use crate::error::Result;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Add, DivAssign, Mul};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    pub const fn new(re: f64, im: f64) -> Self { Self { re, im } }
    pub const fn conj(self) -> Self { Self { re: self.re, im: -self.im } }
    pub fn norm(self) -> f64 { self.re * self.re + self.im * self.im }
    pub fn abs(self) -> f64 { self.norm().sqrt() }
}

impl Add for Complex64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Mul for Complex64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.re * rhs.re - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re)
    }
}

impl DivAssign<f64> for Complex64 {
    fn div_assign(&mut self, rhs: f64) {
        self.re /= rhs;
        self.im /= rhs;
    }
}

impl Display for Complex64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}i", self.re, if self.im < 0.0 { "" } else { "+" }, self.im)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Qubit {
    pub alpha: Complex64,
    pub beta: Complex64,
}

impl Default for Qubit {
    fn default() -> Self {
        Self { alpha: Complex64::new(1.0, 0.0), beta: Complex64::new(0.0, 0.0) }
    }
}

impl Qubit {
    /// C++ equivalent: `Qubit::Qubit(QComplex a, QComplex b)`.
    pub fn new(alpha: Complex64, beta: Complex64) -> Self {
        let mut q = Self { alpha, beta };
        q.normalize();
        q
    }

    /// C++ equivalent: `Qubit::normalize`.
    pub fn normalize(&mut self) {
        let mag = (self.alpha.norm() + self.beta.norm()).sqrt();
        if mag != 0.0 {
            self.alpha /= mag;
            self.beta /= mag;
        }
    }

    /// C++ equivalent: `Qubit::asArray`.
    pub fn as_array(&self) -> [Complex64; 2] { [self.alpha, self.beta] }

    /// C++ equivalent: `Qubit::toString`.
    pub fn to_state_string(&self) -> String {
        format!("({})|0> + ({})|1>", self.alpha, self.beta)
    }
}

/// C++ equivalent: `getPauliX`.
pub fn get_pauli_x() -> [[Complex64; 2]; 2] {
    [
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
    ]
}

/// C++ equivalent: `applyGate`.
pub fn apply_gate(gate: [[Complex64; 2]; 2], state: [Complex64; 2]) -> [Complex64; 2] {
    [
        gate[0][0] * state[0] + gate[0][1] * state[1],
        gate[1][0] * state[0] + gate[1][1] * state[1],
    ]
}

/// C++ equivalent: `QuantumEngine::encodeTextAsQubit`.
pub fn encode_text_as_qubit(text: &str) -> Qubit {
    let mut re = 0.0;
    let mut im = 0.0;
    for (i, byte) in text.bytes().enumerate() {
        re += byte as f64 * (i as f64).cos();
        im += byte as f64 * (i as f64).sin();
    }
    Qubit::new(Complex64::new(re, im), Complex64::new(im, re))
}

/// C++ equivalent: `QuantumEngine::similarity`.
pub fn similarity(a: &Qubit, b: &Qubit) -> f64 {
    let aa = a.as_array();
    let bb = b.as_array();
    let dot = aa[0].conj() * bb[0] + aa[1].conj() * bb[1];
    let amp = dot.abs();
    amp * amp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HilbertCoord { pub x: u32, pub y: u32, pub z: u32 }

/// C++ equivalent: `PrimeHilbertIndexer::tokenize`.
pub fn tokenize(content: &str) -> Vec<String> {
    content.split_whitespace().map(ToOwned::to_owned).collect()
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 1;
    }
    true
}

/// C++ equivalent: `PrimeHilbertIndexer::primeIndices`.
pub fn prime_indices(tokens: &[String]) -> Vec<u64> {
    (0..tokens.len()).filter_map(|i| if is_prime(i as u64) { Some(i as u64) } else { None }).collect()
}

/// C++ equivalent: `PrimeHilbertIndexer::toHilbert3D`.
pub fn to_hilbert_3d(index: u64) -> HilbertCoord {
    HilbertCoord {
        x: ((index >> 0) & 0xFF) as u32,
        y: ((index >> 8) & 0xFF) as u32,
        z: ((index >> 16) & 0xFF) as u32,
    }
}

/// C++ equivalent: `PrimeHilbertIndexer::hashFileToHilbert`.
pub fn hash_file_to_hilbert(path: impl AsRef<Path>) -> Result<HilbertCoord> {
    let content = fs::read_to_string(path)?;
    let tokens = tokenize(&content);
    let sum: u64 = prime_indices(&tokens).into_iter().map(|i| i * i).sum();
    Ok(to_hilbert_3d(sum))
}
