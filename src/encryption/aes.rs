use std::convert::TryInto;

const AES_BLOCK_SIZE : usize = 16;

struct Aes {
  key : [u8; 32], num_rounds : usize,
}

impl Aes {
  pub fn new (key : &[u8])->Aes {
    let key_len = key.len();
    let num_rounds = match key_len{
        16 = > 10,
        24 = > 12,
        32 = > 14,
        _ = > panic !("Invalid key size"),
    };
    let mut aes_key = [0u8; 32];
    aes_key[..key_len].copy_from_slice(key);
    Aes {
    key:
      aes_key, num_rounds : num_rounds,
    }
  }

  fn sub_bytes(&self, state : &mut[u8]) {
        for
          byte in state { *byte = S_BOX[usize::from(*byte)]; }
  }

  fn shift_rows(&self, state : &mut[u8]) {
    let rows = state.chunks_exact_mut(4);
    for (i, row) in rows.enumerate() {
            for
              _ in 0..i { row.rotate_left(1); }
      }
  }

  fn mix_columns(&self, state : &mut[u8]) {
        for
          col in(0..AES_BLOCK_SIZE).step_by(4) {
            let mut temp = [0u8; 4];
            temp.copy_from_slice(&state[col..col + 4]);
            state[col] = gf_mul(0x02, temp[0]) ^ gf_mul(0x03, temp[1]) ^
                         temp[2] ^ temp[3];
            state[col + 1] = temp[0] ^ gf_mul(0x02, temp[1]) ^
                             gf_mul(0x03, temp[2]) ^ temp[3];
            state[col + 2] = temp[0] ^ temp[1] ^ gf_mul(0x02, temp[2]) ^
                             gf_mul(0x03, temp[3]);
            state[col + 3] = gf_mul(0x03, temp[0]) ^ temp[1] ^ temp[2] ^
                             gf_mul(0x02, temp[3]);
          }
  }

  fn add_round_key(&self, state : &mut[u8], round_key : &[u8]) {
    for (state_byte, key_byte) in state.iter_mut().zip(round_key.iter()) {
        *state_byte ^= key_byte;
      }
  }

  fn expand_key(&self)->Vec<u8> {
    let mut expanded_key =
        Vec::with_capacity(4 * AES_BLOCK_SIZE * (self.num_rounds + 1));
    let key_bytes = self.key.len();
    let mut temp = [0u8; 4];

        for
          i in 0..key_bytes { expanded_key.push(self.key[i]); }

        for
          i in key_bytes..4 * AES_BLOCK_SIZE*(self.num_rounds + 1) {
            temp.copy_from_slice(&expanded_key[i - 4..i]);

            if i
              % key_bytes == 0 {
                temp.rotate_left(1);
                self.sub_bytes(&mut temp);
                let rcon_index = i / key_bytes;
                temp[0] ^= RCON[rcon_index];
              }
            else if key_bytes
              > 24 && i % key_bytes == 16 { self.sub_bytes(&mut temp); }

        for
          j in 0..4 {
            let byte = expanded_key[i - key_bytes * 4 + j];
            temp[j] ^= byte;
            expanded_key.push(temp[j]);
          }
          }

        expanded_key
  }

  fn encrypt_block(&self, block: &[u8], expanded_key: &[u8]) -> [u8; AES_BLOCK_SIZE] {
    let mut state = [0u8; AES_BLOCK_SIZE];
    state.copy_from_slice(block);

    let mut round_key_offset = 0;
    self.add_round_key(&mut state, &expanded_key[round_key_offset..round_key_offset + AES_BLOCK_SIZE]);
    round_key_offset += AES_BLOCK_SIZE;

    for _ in 0..self.num_rounds - 1 {
        self.sub_bytes(&mut state);
        self.shift_rows(&mut state);
        self.mix_columns(&mut state);
        self.add_round_key(&mut state, &expanded_key[round_key_offset..round_key_offset + AES_BLOCK_SIZE]);
        round_key_offset += AES_BLOCK_SIZE;
    }

    self.sub_bytes(&mut state);
    self.shift_rows(&mut state);
    self.add_round_key(&mut state, &expanded_key[round_key_offset..round_key_offset + AES_BLOCK_SIZE]);

    state.try_into().unwrap()
}

fn decrypt_block(&self, block: &[u8], expanded_key: &[u8]) -> [u8; AES_BLOCK_SIZE] {
    let mut state = [0u8; AES_BLOCK_SIZE];
    state.copy_from_slice(block);

    let mut round_key_offset = AES_BLOCK_SIZE * self.num_rounds;
    self.add_round_key(&mut state, &expanded_key[round_key_offset - AES_BLOCK_SIZE..round_key_offset]);
    round_key_offset -= AES_BLOCK_SIZE;

    for _ in 0..self.num_rounds - 1 {
        self.inv_shift_rows(&mut state);
        self.inv_sub_bytes(&mut state);
        self.add_round_key(&mut state, &expanded_key[round_key_offset - AES_BLOCK_SIZE..round_key_offset]);
        round_key_offset -= AES_BLOCK_SIZE;
        self.inv_mix_columns(&mut state);
    }

    self.inv_shift_rows(&mut state);
    self.inv_sub_bytes(&mut state);
    self.add_round_key(&mut state, &expanded_key[0..AES_BLOCK_SIZE]);

    state.try_into().unwrap()
}

fn inv_sub_bytes(&self, state: &mut [u8]) {
    for
      byte in state { *byte = INV_S_BOX[usize::from(*byte)]; }
  }

  fn inv_shift_rows(&self, state : &mut[u8]) {
    let rows = state.chunks_exact_mut(4);
    for (i, row) in rows.enumerate() {
        for
          _ in 0..i { row.rotate_right(1); }
      }
  }

  fn inv_mix_columns(&self, state : &mut[u8]) {
    for
      col in(0..AES_BLOCK_SIZE).step_by(4) {
        let mut temp = [0u8; 4];
        temp.copy_from_slice(&state[col..col + 4]);
        state[col] = gf_mul(0x0e, temp[0]) ^ gf_mul(0x0b, temp[1]) ^
                     gf_mul(0x0d, temp[2]) ^ gf_mul(0x09, temp[3]);
        state[col + 1] = gf_mul(0x09, temp[0]) ^ gf_mul(0x0e, temp[1]) ^
                         gf_mul(0x0b, temp[2]) ^ gf_mul(0x0d, temp[3]);
        state[col + 2] = gf_mul(0x0d, temp[0]) ^ gf_mul(0x09, temp[1]) ^
                         gf_mul(0x0e, temp[2]) ^ gf_mul(0x0b, temp[3]);
        state[col + 3] = gf_mul(0x0b, temp[0]) ^ gf_mul(0x0d, temp[1]) ^
                         gf_mul(0x09, temp[2]) ^ gf_mul(0x0e, temp[3]);
      }
  }

  fn gf_mul(a : u8, b : u8)->u8 {
    let mut p = 0;
    let mut a = a as u32;
    let mut b = b as u32;
    while
      b != 0 {
        if b
          &1 == 1 { p ^= a; }
        let hi_bit_set = (a & 0x80) == 0x80;
        a <<= 1;
        if hi_bit_set {
          a ^= 0x1b;
        }
        b >>= 1;
      }
    p as u8
  }

#[cfg(test)]
  mod tests {
    use super::*;
#[test]
    fn test_encrypt_decrypt() {
      let aes = Aes::new (&[0u8; AES_BLOCK_SIZE]);
      let plaintext = [0u8; AES_BLOCK_SIZE];
      let expanded_key = aes.expand_key(&[0u8; AES_KEY_SIZE]);
      let ciphertext = aes.encrypt_block(&plaintext, &expanded_key);
      assert_eq !(plaintext, aes.decrypt_block(&ciphertext, &expanded_key));
    }
  }
