use sha2::{Sha256, Digest};


// Language: rust
// Path: src/hash.rs
// This calculates the hash of a string.
pub fn get_sha256_hash(input_text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input_text.as_bytes());
    let result = hasher.finalize();
    return format!("{:x}", result);
}