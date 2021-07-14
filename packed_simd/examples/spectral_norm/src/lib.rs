//! Spectral Norm
#![deny(rust_2018_idioms)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(
    clippy::cast_precision_loss,
    clippy::must_use_candidate
)]

pub mod scalar;
pub mod simd;

fn A(i: usize, j: usize) -> f64 {
    ((i + j) * (i + j + 1) / 2 + i + 1) as f64
}

pub fn spectral_norm(n: usize, alg: usize) -> f64 {
    match alg {
        0 => simd::spectral_norm(n),
        1 => scalar::spectral_norm(n),
        v => panic!("unknown algorithm value: {}", v),
    }
}
