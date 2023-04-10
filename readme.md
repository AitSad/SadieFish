# SadieFish

SadieFish is a cryptography library written in Rust. It provides a set of encryption algorithms and utilities for use in applications that require secure data storage and transmission.

## Features

- Caesar cipher
- Vigenère cipher
- Advanced Encryption Standard (AES)
- Data Encryption Standard (DES)
- Rivest–Shamir–Adleman (RSA)
- Base64 encoding/decoding
- Hexadecimal encoding/decoding
- Random number generation

## Installation

To use SadieFish in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
SadieFish = { git = "https://github.com/AitSad/SadieFish" }
```

## Usage
To use SadieFish in your Rust code, import the necessary modules:

```rust
use SadieFish::encryption::{caesar, vigenere, aes, des, rsa};
use SadieFish::utils::{base64, hex, rand};
```

Then, call the desired encryption or utility function. For example, to encrypt a message using the Caesar cipher:

```rust
let message = "Hello, world!";
let key = 3;
let encrypted = caesar::encrypt(message, key);
println!("Encrypted message: {}", encrypted);
```
