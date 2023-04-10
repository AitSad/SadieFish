const HEX_ALPHABET: &[u8; 16] = b"0123456789abcdef";

fn encode_hex(input: &[u8]) -> String {
    let mut result = String::new();
    for &byte in input {
        result.push(HEX_ALPHABET[(byte >> 4) as usize] as char);
        result.push(HEX_ALPHABET[(byte & 0xf) as usize] as char);
    }
    result
}

fn decode_hex(input: &str) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut chars = input.chars().fuse();
    loop {
        let hi = match chars.next() {
            Some(c) => HEX_ALPHABET.iter().position(|&b| b as char == c).ok_or("invalid character in input")?,
            None => break,
        };
        let lo = match chars.next() {
            Some(c) => HEX_ALPHABET.iter().position(|&b| b as char == c).ok_or("invalid character in input")?,
            None => return Err("odd number of characters in input".to_owned()),
        };
        output.push((hi << 4 | lo) as u8);
    }
    Ok(output)
}
