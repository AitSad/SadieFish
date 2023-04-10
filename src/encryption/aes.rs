use aes::Aes256;
use aes::block_cipher_trait::BlockCipher;
use aes::cipher::KeyInit;

pub fn aes_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(key);
    let mut ciphertext = plaintext.to_vec();
    cipher.encrypt_block(&mut ciphertext);
    ciphertext
}

pub fn aes_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(key);
    let mut plaintext = ciphertext.to_vec();
    cipher.decrypt_block(&mut plaintext);
    plaintext
}

pub fn aes_cbc_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(key);
    let mut ciphertext = plaintext.to_vec();
    let mut prev_block = iv.to_vec();
    for block in ciphertext.chunks_mut(16) {
        for i in 0..16 {
            block[i] ^= prev_block[i];
        }
        cipher.encrypt_block(block);
        prev_block.copy_from_slice(block);
    }
    ciphertext
}

pub fn aes_cbc_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(key);
    let mut plaintext = ciphertext.to_vec();
    let mut prev_block = iv.to_vec();
    for block in plaintext.chunks_mut(16) {
        let temp_block = block.to_vec();
        cipher.decrypt_block(block);
        for i in 0..16 {
            block[i] ^= prev_block[i];
        }
        prev_block.copy_from_slice(&temp_block);
    }
    plaintext
}

pub fn aes_ctr(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(key);
    let mut counter = nonce.to_vec();
    let mut ciphertext = Vec::new();
    for (i, block) in plaintext.chunks(16).enumerate() {
        let mut keystream_block = counter.to_vec();
        cipher.encrypt_block(&mut keystream_block);
        let mut ciphertext_block = block.to_vec();
        for j in 0..16 {
            ciphertext_block[j] ^= keystream_block[j];
        }
        ciphertext.extend_from_slice(&ciphertext_block);
        counter[15] = i as u8 + 1; // increment counter
    }
    ciphertext
}
