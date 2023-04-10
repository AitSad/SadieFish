use sadiefish::{encrypt, decrypt};

#[test]
fn test_encryption() {
    let plaintext = "Hello, world!";
    let key = "supersecretkey";
    
    let ciphertext = encrypt(plaintext.as_bytes(), key.as_bytes()).unwrap();
    assert_ne!(plaintext.as_bytes(), &ciphertext[..]);
    
    let decrypted = decrypt(&ciphertext[..], key.as_bytes()).unwrap();
    assert_eq!(plaintext.as_bytes(), &decrypted[..]);
}

#[test]
fn test_invalid_key() {
    let plaintext = "Hello, world!";
    let key = "shortkey";
    
    let ciphertext = encrypt(plaintext.as_bytes(), key.as_bytes());
    assert!(ciphertext.is_err());
    
    let decrypted = decrypt(&ciphertext.unwrap()[..], key.as_bytes());
    assert!(decrypted.is_err());
}

#[test]
fn test_invalid_input() {
    let plaintext = "Hello, world!";
    let key = "supersecretkey";
    
    let ciphertext = encrypt(&[0u8; 100], key.as_bytes());
    assert!(ciphertext.is_err());
    
    let decrypted = decrypt(&ciphertext.unwrap()[..], key.as_bytes());
    assert!(decrypted.is_err());
}
