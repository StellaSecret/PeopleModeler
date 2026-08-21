use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit},
};

#[allow(dead_code)]
const KEY_SESSION_KEY: &str = "pm_enc_key";

#[cfg(target_arch = "wasm32")]
fn load_or_create_key() -> [u8; 32] {
    use gloo_storage::Storage;
    let key_hex: Option<String> = gloo_storage::LocalStorage::get(KEY_SESSION_KEY).ok();
    if let Some(h) = key_hex {
        if h.len() == 64 {
            if let Some(raw) = hex_decode(&h) {
                let mut k = [0u8; 32];
                k.copy_from_slice(&raw);
                return k;
            }
        }
    }
    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key).expect("os rng");
    let _ = gloo_storage::LocalStorage::set(KEY_SESSION_KEY, hex_encode(&key));
    key
}

#[cfg(target_arch = "wasm32")]
pub fn encrypt(plain: &[u8]) -> Vec<u8> {
    let key_bytes = load_or_create_key();
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let mut nonce_bytes = [0u8; 12];
    getrandom::getrandom(&mut nonce_bytes).expect("nonce rng");
    let nonce = Nonce::from_slice(&nonce_bytes);
    let mut out = cipher.encrypt(nonce, plain).expect("encrypt");
    let mut result = Vec::with_capacity(12 + out.len());
    result.extend_from_slice(&nonce_bytes);
    result.append(&mut out);
    result
}

#[cfg(target_arch = "wasm32")]
pub fn decrypt(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 28 {
        return None;
    }
    let key_bytes = load_or_create_key();
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ciphertext).ok()
}

#[allow(dead_code)]
pub fn derive_key(passphrase: &str, salt: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(passphrase.as_bytes());
    hasher.update(salt);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

#[allow(dead_code)]
pub fn encrypt_with_passphrase(plain: &[u8], passphrase: &str) -> Vec<u8> {
    let mut salt = [0u8; 16];
    getrandom::getrandom(&mut salt).expect("salt rng");
    let key_bytes = derive_key(passphrase, &salt);
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let mut nonce_bytes = [0u8; 12];
    getrandom::getrandom(&mut nonce_bytes).expect("nonce rng");
    let nonce = Nonce::from_slice(&nonce_bytes);
    let mut ct = cipher.encrypt(nonce, plain).expect("encrypt");
    let mut result = Vec::with_capacity(1 + 16 + 12 + ct.len());
    result.push(16);
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.append(&mut ct);
    result
}

#[allow(dead_code)]
pub fn decrypt_with_passphrase(data: &[u8], passphrase: &str) -> Option<Vec<u8>> {
    if data.len() < 30 {
        return None;
    }
    let salt_len = data[0] as usize;
    if 1 + salt_len + 12 + 16 > data.len() {
        return None;
    }
    let salt = &data[1..1 + salt_len];
    let nonce_bytes = &data[1 + salt_len..1 + salt_len + 12];
    let ciphertext = &data[1 + salt_len + 12..];
    let key_bytes = derive_key(passphrase, salt);
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ciphertext).ok()
}

#[allow(dead_code)]
fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

#[allow(dead_code)]
fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use super::*;
    use gloo_storage::Storage;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn load_or_create_key_reuses_stored_key() {
        let key = [0x42u8; 32];
        gloo_storage::LocalStorage::set(KEY_SESSION_KEY, hex_encode(&key)).unwrap();
        let loaded = load_or_create_key();
        assert_eq!(loaded, key);
    }

    #[wasm_bindgen_test]
    fn hex_encode_decode_roundtrip() {
        let data = vec![0u8, 1, 127, 128, 255, 42];
        let encoded = hex_encode(&data);
        assert_eq!(encoded, "00017f80ff2a");
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[wasm_bindgen_test]
    fn hex_decode_odd_length_returns_none() {
        assert!(hex_decode("abc").is_none());
    }

    #[wasm_bindgen_test]
    fn hex_decode_empty() {
        let result = hex_decode("").unwrap();
        assert!(result.is_empty());
    }

    #[wasm_bindgen_test]
    fn hex_decode_invalid_chars() {
        assert!(hex_decode("zzzz").is_none());
    }

    #[wasm_bindgen_test]
    fn derive_key_deterministic() {
        let k1 = derive_key("passphrase", b"salt123");
        let k2 = derive_key("passphrase", b"salt123");
        assert_eq!(k1, k2);
    }

    #[wasm_bindgen_test]
    fn derive_key_different_passphrases_differ() {
        let k1 = derive_key("pass1", b"salt");
        let k2 = derive_key("pass2", b"salt");
        assert_ne!(k1, k2);
    }

    #[wasm_bindgen_test]
    fn derive_key_different_salts_differ() {
        let k1 = derive_key("pass", b"salt1");
        let k2 = derive_key("pass", b"salt2");
        assert_ne!(k1, k2);
    }

    #[wasm_bindgen_test]
    fn encrypt_decrypt_roundtrip() {
        let plain = b"hello world test data 1234567890";
        let enc = encrypt(plain);
        assert_ne!(enc, plain.to_vec());
        assert!(enc.len() >= 28);
        let dec = decrypt(&enc).unwrap();
        assert_eq!(dec, plain.to_vec());
    }

    #[wasm_bindgen_test]
    fn decrypt_too_short_returns_none() {
        assert!(decrypt(&[0u8; 10]).is_none());
    }

    #[wasm_bindgen_test]
    fn decrypt_garbage_returns_none() {
        let garbage = vec![255u8; 64];
        assert!(decrypt(&garbage).is_none());
    }

    #[wasm_bindgen_test]
    fn encrypt_with_passphrase_decrypt_roundtrip() {
        let plain = b"secret data with passphrase";
        let pp = "my_secret_pass";
        let enc = encrypt_with_passphrase(plain, pp);
        assert!(enc.len() >= 30);
        let dec = decrypt_with_passphrase(&enc, pp).unwrap();
        assert_eq!(dec, plain.to_vec());
    }

    #[wasm_bindgen_test]
    fn decrypt_with_passphrase_wrong_passphrase_returns_none() {
        let plain = b"protected data";
        let enc = encrypt_with_passphrase(plain, "correct");
        let result = decrypt_with_passphrase(&enc, "wrong");
        assert!(result.is_none());
    }

    #[wasm_bindgen_test]
    fn decrypt_with_passphrase_too_short_returns_none() {
        assert!(decrypt_with_passphrase(&[0u8; 10], "pp").is_none());
    }

    #[wasm_bindgen_test]
    fn decrypt_with_passphrase_bad_length_returns_none() {
        // salt_len=200, way larger than data
        let data = vec![200u8, 0, 0, 0, 0];
        assert!(decrypt_with_passphrase(&data, "pp").is_none());
    }

    #[wasm_bindgen_test]
    fn encrypt_produces_different_ciphertext_each_time() {
        let plain = b"same input";
        let enc1 = encrypt(plain);
        let enc2 = encrypt(plain);
        assert_ne!(enc1, enc2);
    }

    #[wasm_bindgen_test]
    fn encrypt_with_passphrase_produces_different_ciphertext_each_time() {
        let plain = b"same input";
        let pp = "passphrase";
        let enc1 = encrypt_with_passphrase(plain, pp);
        let enc2 = encrypt_with_passphrase(plain, pp);
        assert_ne!(enc1, enc2);
    }

    #[wasm_bindgen_test]
    fn encrypt_empty_plaintext() {
        let enc = encrypt(b"");
        let dec = decrypt(&enc).unwrap();
        assert!(dec.is_empty());
    }

    #[wasm_bindgen_test]
    fn encrypt_with_passphrase_empty_plaintext() {
        let enc = encrypt_with_passphrase(b"", "pp");
        let dec = decrypt_with_passphrase(&enc, "pp").unwrap();
        assert!(dec.is_empty());
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn hex_encode_decode_roundtrip() {
        let data = vec![0u8, 1, 127, 128, 255, 42];
        let encoded = hex_encode(&data);
        assert_eq!(encoded, "00017f80ff2a");
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn hex_decode_odd_length_returns_none() {
        assert!(hex_decode("abc").is_none());
    }

    #[test]
    fn hex_decode_empty() {
        let result = hex_decode("").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn hex_decode_invalid_chars() {
        assert!(hex_decode("zzzz").is_none());
    }

    #[test]
    fn derive_key_deterministic() {
        let k1 = derive_key("passphrase", b"salt123");
        let k2 = derive_key("passphrase", b"salt123");
        assert_eq!(k1, k2);
    }

    #[test]
    fn derive_key_different_passphrases_differ() {
        let k1 = derive_key("pass1", b"salt");
        let k2 = derive_key("pass2", b"salt");
        assert_ne!(k1, k2);
    }

    #[test]
    fn derive_key_different_salts_differ() {
        let k1 = derive_key("pass", b"salt1");
        let k2 = derive_key("pass", b"salt2");
        assert_ne!(k1, k2);
    }

    #[test]
    fn encrypt_with_passphrase_decrypt_roundtrip() {
        let plain = b"secret data with passphrase";
        let pp = "my_secret_pass";
        let enc = encrypt_with_passphrase(plain, pp);
        assert!(enc.len() >= 30);
        let dec = decrypt_with_passphrase(&enc, pp).unwrap();
        assert_eq!(dec, plain.to_vec());
    }

    #[test]
    fn decrypt_with_passphrase_wrong_passphrase_returns_none() {
        let plain = b"protected data";
        let enc = encrypt_with_passphrase(plain, "correct");
        let result = decrypt_with_passphrase(&enc, "wrong");
        assert!(result.is_none());
    }

    #[test]
    fn decrypt_with_passphrase_too_short_returns_none() {
        assert!(decrypt_with_passphrase(&[0u8; 10], "pp").is_none());
    }

    #[test]
    fn decrypt_with_passphrase_bad_length_returns_none() {
        let data = vec![200u8, 0, 0, 0, 0];
        assert!(decrypt_with_passphrase(&data, "pp").is_none());
    }

    #[test]
    fn encrypt_with_passphrase_produces_different_ciphertext_each_time() {
        let plain = b"same input";
        let pp = "passphrase";
        let enc1 = encrypt_with_passphrase(plain, pp);
        let enc2 = encrypt_with_passphrase(plain, pp);
        assert_ne!(enc1, enc2);
    }

    #[test]
    fn encrypt_with_passphrase_empty_plaintext() {
        let enc = encrypt_with_passphrase(b"", "pp");
        let dec = decrypt_with_passphrase(&enc, "pp").unwrap();
        assert!(dec.is_empty());
    }

    #[test]
    fn hex_encode_all_byte_values() {
        let data: Vec<u8> = (0..=255).collect();
        let encoded = hex_encode(&data);
        assert_eq!(encoded.len(), 512);
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn derive_key_zero_salt() {
        let k = derive_key("test", &[0u8; 32]);
        assert_ne!(k, [0u8; 32]);
    }
}
