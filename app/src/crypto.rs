use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit},
};

const KEY_SESSION_KEY: &str = "pm_enc_key";

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

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}
