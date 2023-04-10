use std::io;

// Function to encrypt a message using the Caesar cipher
fn caesar_encrypt(plaintext: &str, shift: u8) -> String {
    let mut ciphertext = String::new();
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let index = (c as u8 - base + shift) % 26;
            ciphertext.push((base + index) as char);
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

// Function to decrypt a message using the Caesar cipher
fn caesar_decrypt(ciphertext: &str, shift: u8) -> String {
    caesar_encrypt(ciphertext, 26 - shift)
}

fn main() {
    println!("Enter a message to encrypt:");
    let mut plaintext = String::new();
    io::stdin().read_line(&mut plaintext).expect("Failed to read line");

    // Convert the shift to a u8 value
    let shift: u8 = 3;

    let ciphertext = caesar_encrypt(&plaintext, shift);
    println!("Encrypted message: {}", ciphertext);

    let decrypted = caesar_decrypt(&ciphertext, shift);
    println!("Decrypted message: {}", decrypted);
}
