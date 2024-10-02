use hex;
use once_cell::sync::Lazy;

/// The "Nothing Up My Sleeve" (NUMS) point.
pub static SECP256K1_GENERATOR: Lazy<Vec<u8>> = Lazy::new(|| {
    hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap()
});

/// The counter example.
pub mod counter;
