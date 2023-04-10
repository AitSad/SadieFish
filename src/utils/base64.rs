const BASE64_ALPHABET : &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

                        fn
                        encode_base64(input
                                      : &[u8])
                            ->String {
  let mut result = String::new ();
  let mut i = 0;
  let len = input.len();

  while
    i < len {
      let value = input[i];
      i += 1;

      result.push(BASE64_ALPHABET[(value >> 2) as usize]);
      if i
        == len {
          result.push(BASE64_ALPHABET[((value & 0x03) << 4) as usize]);
          result.push_str("==");
          break;
        }

      let value = (value << 8) | input[i];
      i += 1;

      result.push(BASE64_ALPHABET[((value >> 6) & 0x3F) as usize]);
      result.push(BASE64_ALPHABET[(value & 0x3F) as usize]);

      if i
        == len {
          result.push(BASE64_ALPHABET[((value & 0x0F) << 2) as usize]);
          result.push('=');
          break;
        }

      let value = (value << 8) | input[i];
      i += 1;

      result.push(BASE64_ALPHABET[((value >> 12) & 0x3F) as usize]);
      result.push(BASE64_ALPHABET[((value >> 6) & 0x3F) as usize]);
      result.push(BASE64_ALPHABET[(value & 0x3F) as usize]);
    }

  result
}

fn decode_base64(input : &str)->Result<Vec<u8>, String> {
  let input = input.as_bytes();
  let mut output = Vec::new ();

  let mut i = 0;
  let len = input.len();

  while
    i < len {
      let value =
          BASE64_ALPHABET.iter().position(| &c | c == input[i]).unwrap_or(0xFF);
      i += 1;
      if value
        == 0xFF { return Err("invalid character in input".to_owned()); }

      let mut v = (value << 18) as u32;

      if i
        == len { return Err("invalid input length".to_owned()); }

      let value =
          BASE64_ALPHABET.iter().position(| &c | c == input[i]).unwrap_or(0xFF);
      i += 1;
      if value
        == 0xFF { return Err("invalid character in input".to_owned()); }

      v |= (value << 12) as u32;

      if i
        == len || input[i] == b '=' {
          output.push(((v >> 16) & 0xFF) as u8);
          break;
        }

      let value =
          BASE64_ALPHABET.iter().position(| &c | c == input[i]).unwrap_or(0xFF);
      i += 1;
      if value
        == 0xFF { return Err("invalid character in input".to_owned()); }

      v |= (value << 6) as u32;

      if i
        == len || input[i] == b '=' {
          output.push(((v >> 16) & 0xFF) as u8);
          output.push(((v >> 8) & 0xFF) as u8);
          break;
        }
      let value =
          BASE64_ALPHABET.iter().position(| &c | c == input[i]).unwrap_or(0xFF);
      i += 1;
      if value
        == 0xFF { return Err("invalid character in input".to_owned()); }

      v |= value as u32;

      output.push(((v >> 16) & 0xFF) as u8);
      output.push(((v >> 8) & 0xFF) as u8);
      output.push((v & 0xFF) as u8);
    }

  Ok(output)
}

// Test cases
#[cfg(test)]
mod tests {
  use super::*;
#[test]
  fn test_encode_base64() {
    assert_eq !(encode_base64(b ""), "");
    assert_eq !(encode_base64(b "f"), "Zg==");
    assert_eq !(encode_base64(b "fo"), "Zm8=");
    assert_eq !(encode_base64(b "foo"), "Zm9v");
    assert_eq !(encode_base64(b "foob"), "Zm9vYg==");
    assert_eq !(encode_base64(b "fooba"), "Zm9vYmE=");
    assert_eq !(encode_base64(b "foobar"), "Zm9vYmFy");
  }

#[test]
  fn test_decode_base64() {
    assert_eq !(decode_base64(""), Ok(vec ![]));
    assert_eq !(decode_base64("Zg=="), Ok(vec ![b 'f']));
    assert_eq !(decode_base64("Zm8="), Ok(vec ![ b 'f', b 'o' ]));
    assert_eq !(decode_base64("Zm9v"), Ok(vec ![ b 'f', b 'o', b 'o' ]));
    assert_eq !(decode_base64("Zm9vYg=="),
                Ok(vec ![ b 'f', b 'o', b 'o', b 'b' ]));
    assert_eq !(decode_base64("Zm9vYmE="),
                Ok(vec ![ b 'f', b 'o', b 'o', b 'b', b 'a' ]));
    assert_eq !(decode_base64("Zm9vYmFy"),
                Ok(vec ![ b 'f', b 'o', b 'o', b 'b', b 'a', b 'r' ]));
    assert !(decode_base64("A").is_err());
    assert !(decode_base64("====").is_err());
  }
}
