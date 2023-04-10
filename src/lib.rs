pub mod encryption {
    pub mod caesar;
    pub mod vigenere;
    pub mod aes;
    pub mod des;
    pub mod rsa;
}

pub mod utils {
    pub mod base64;
    pub mod hex;
    pub mod bit_ops;
    pub mod rand;
}

#[cfg(test)]
mod tests {
    mod utils_tests;
    mod encryption_tests;
}
