use aes_gcm_siv::aead::KeyInit;
use sha3::Digest;
use aes_gcm_siv::aead::Aead;
use anyhow::Result;
use anyhow::Error;
use anyhow::bail;


pub fn encrypt(plaintext: &[u8], key: &str) -> Result<Vec<u8>> {
    // Create sha3 hash from key to enforce 256 bit key length
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(key.as_bytes());
    let key = hasher.finalize();

    // Generate nonce (96 bits)
    let mut nonce = [0u8; 12];
    getrandom::getrandom(&mut nonce).map_err(Error::msg)?;

    // Generate cipher (aes_gcm_siv)
    let cipher = aes_gcm_siv::Aes256GcmSiv::new(&key);

    // Encrypt plaintext
    let mut ciphertext = cipher.encrypt(&nonce.into(), plaintext.as_ref()).map_err(Error::msg)?;
    // Extend ciphertext with nonce
    ciphertext.extend(nonce);
    Ok(ciphertext)
}

pub fn decrypt(ciphertext: &[u8], key: &str) -> Result<Vec<u8>> {
    // Check if ciphertext is long enough to contain ciphertext+nonce
    if ciphertext.len() < 12 {
        bail!("Ciphertext to short");
    }

    // Create sha3 hash from key to enforce 256 bit key length
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(key.as_bytes());
    let key = hasher.finalize();

    let nonce_start = ciphertext.len() - 12;
    let nonce: [u8; 12] = ciphertext[nonce_start..].try_into().map_err(Error::msg)?;

    let actual_ciphertext = &ciphertext[..nonce_start];

    // Generate cipher (aes_gcm_siv)
    let cipher = aes_gcm_siv::Aes256GcmSiv::new(&key);

    cipher.decrypt(&nonce.into(), actual_ciphertext.as_ref()).map_err(Error::msg)
}
