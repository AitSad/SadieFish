/// Encrypts a plaintext message using the Vigenere cipher with the given key.
pub fn encrypt(plaintext: &str, key: &str) -> String {
    let plaintext_bytes = plaintext.as_bytes();
    let key_bytes = key.as_bytes();
    let mut ciphertext = String::new();
    let mut key_index = 0;

    for &byte in plaintext_bytes {
        let key_byte = key_bytes[key_index % key_bytes.len()];
        let cipher_byte = (byte + key_byte) % u8::MAX + 1;
        ciphertext.push(cipher_byte as char);
        key_index += 1;
    }

    ciphertext
}

/// Decrypts a ciphertext message using the Vigenere cipher with the given key.
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let ciphertext_bytes = ciphertext.as_bytes();
    let key_bytes = key.as_bytes();
    let mut plaintext = String::new();
    let mut key_index = 0;

    for &byte in ciphertext_bytes {
        let key_byte = key_bytes[key_index % key_bytes.len()];
        let plain_byte = (byte + u8::MAX + 1 - key_byte) % u8::MAX + 1;
        plaintext.push(plain_byte as char);
        key_index += 1;
    }

    plaintext
}
