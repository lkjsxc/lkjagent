use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FingerprintError {
    pub message: String,
}

pub fn stable_fingerprint<T: Serialize>(value: &T) -> Result<String, FingerprintError> {
    match serde_json::to_vec(value) {
        Ok(bytes) => Ok(format!("fnv1a64:{:016x}", fnv1a64(&bytes))),
        Err(err) => Err(FingerprintError {
            message: err.to_string(),
        }),
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x00000100000001b3;
    let mut hash = OFFSET;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}
