use base64::Engine;
use sha2::Digest;

const BCRYPT_COST: u32 = 10;

/// Generate a hash for the provided input.
/// The returned String is a hash of the input and it's salt
///
/// # Panics
///
/// When the provided `salt` is not exactly 16 characters long
///
/// # Errors
///
/// If hashing fails
///
pub fn hash(input: &str, salt: &str, pepper: &str) -> Result<String, bcrypt::BcryptError> {
    let mut hasher = sha2::Sha512_256::new();

    hasher.update(input);
    hasher.update(pepper);

    let salt_bytes = salt.as_bytes();
    if salt_bytes.len() != 16 {
        panic!("Salt is not 16 bytes long")
    }
    let mut salt_bytes_arr = [0_u8; 16];
    salt_bytes_arr.copy_from_slice(salt_bytes);

    let engine = base64::prelude::BASE64_STANDARD;
    let hash = engine.encode(hasher.finalize());
    let bcrypt = bcrypt::hash_with_salt(hash, BCRYPT_COST, salt_bytes_arr)?.format_for_version(bcrypt::Version::TwoB);

    Ok(bcrypt)
}

/// Verify an input is the same as the stored hash. The same `pepper` must be used
///
/// # Errors
///
/// If verifying fails
pub fn verify(stored_hash: &str, input: &str, pepper: &str) -> Result<bool, bcrypt::BcryptError> {
    let mut hasher = sha2::Sha512_256::new();

    hasher.update(input);
    hasher.update(pepper);

    let engine = base64::prelude::BASE64_STANDARD;
    let hash = engine.encode(hasher.finalize());

    let correct = bcrypt::verify(hash, stored_hash)?;

    Ok(correct)
}