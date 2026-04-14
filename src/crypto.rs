use aes_gcm::{
  aead::{Aead, AeadCore, KeyInit, OsRng},
  Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EncryptedPayload {
  nonce: String,
  ciphertext: String,
}

pub fn encrypt_data(data: &str, key_str: &str) -> Result<String, String> {
  let key_bytes = key_str.as_bytes();
  if key_bytes.len() != 32 {
    return Err("Encryption key must be exactly 32 bytes long".to_string());
  }

  let key = Key::<Aes256Gcm>::from_slice(key_bytes);
  let cipher = Aes256Gcm::new(key);

  let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
  let ciphertext = cipher
    .encrypt(&nonce, data.as_ref())
    .map_err(|e| format!("Encryption failure: {}", e))?;

  let payload = EncryptedPayload {
    nonce: STANDARD.encode(nonce),
    ciphertext: STANDARD.encode(ciphertext),
  };

  serde_json::to_string(&payload).map_err(|e| format!("Failed to serialize payload: {}", e))
}

pub fn decrypt_data(encrypted_json: &str, key_str: &str) -> Result<String, String> {
  let key_bytes = key_str.as_bytes();
  if key_bytes.len() != 32 {
    return Err("Decryption key must be exactly 32 bytes long".to_string());
  }

  let payload: EncryptedPayload =
    serde_json::from_str(encrypted_json).map_err(|e| format!("Invalid JSON payload: {}", e))?;

  let nonce_bytes = STANDARD
    .decode(&payload.nonce)
    .map_err(|e| format!("Invalid nonce encoding: {}", e))?;
  let ciphertext_bytes = STANDARD
    .decode(&payload.ciphertext)
    .map_err(|e| format!("Invalid ciphertext encoding: {}", e))?;

  let key = Key::<Aes256Gcm>::from_slice(key_bytes);
  let cipher = Aes256Gcm::new(key);
  let nonce = Nonce::from_slice(&nonce_bytes);

  let plaintext = cipher
    .decrypt(nonce, ciphertext_bytes.as_ref())
    .map_err(|e| format!("Decryption failure: {}", e))?;

  String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8 in decrypted data: {}", e))
}
