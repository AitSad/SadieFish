// SadieFish: unit_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = b"hello world";
        let key = generate_key();

        let ciphertext = encrypt(plaintext, &key).unwrap();
        assert_ne!(ciphertext, plaintext);

        let decrypted = decrypt(&ciphertext, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_with_wrong_key() {
        let plaintext = b"hello world";
        let key = generate_key();

        let ciphertext = encrypt(plaintext, &key).unwrap();
        assert_ne!(ciphertext, plaintext);

        // try decrypting with a different key
        let wrong_key = generate_key();
        let decrypted = decrypt(&ciphertext, &wrong_key);
        assert!(decrypted.is_err());
    }

    #[test]
    fn test_generate_key_randomness() {
        // Generate 100 keys and make sure they're all different
        let mut keys = vec![];
        for _ in 0..100 {
            let key = generate_key();
            assert_eq!(key.len(), 32);
            assert!(!keys.contains(&key));
            keys.push(key);
        }
    }

    #[test]
    fn test_encrypt_with_invalid_key() {
        let plaintext = b"hello world";
        let key = b"shortkey"; // invalid key length

        let encrypted = encrypt(plaintext, key);
        assert!(encrypted.is_err());
    }

    #[test]
    fn test_decrypt_with_invalid_key() {
        let ciphertext = b"invalidciphertext";
        let key = b"shortkey"; // invalid key length

        let decrypted = decrypt(ciphertext, key);
        assert!(decrypted.is_err());
    }

    #[test]
    fn test_encrypt_decrypt_with_empty_input() {
        let plaintext = b"";
        let key = generate_key();

        let ciphertext = encrypt(plaintext, &key).unwrap();
        assert_eq!(ciphertext, plaintext);

        let decrypted = decrypt(&ciphertext, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_with_random_input() {
        let mut rng = rand::thread_rng();

        for i in 0..10 {
            let len = rng.gen_range(0, 1000);
            let mut plaintext = vec![0u8; len];
            rng.fill_bytes(&mut plaintext);

            let key = generate_key();

            let ciphertext = encrypt(&plaintext, &key).unwrap();

            let decrypted = decrypt(&ciphertext, &key).unwrap();
            assert_eq!(decrypted, plaintext, "failed on iteration {}", i);
        }
    }
}
