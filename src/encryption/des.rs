use rand::{prelude::*, rngs::StdRng, SeedableRng};

const PERMUTATION_TABLE : [usize; 64] = [
  16, 7, 20, 21, 29, 12, 28, 17, 1,  15, 23, 26, 5,  18, 31, 10,
  2,  8, 24, 14, 32, 27, 3,  9,  19, 13, 30, 6,  22, 11, 4,  25
];

const INVERSE_PERMUTATION_TABLE : [usize; 64] = [
  9, 17, 23, 31, 13, 28, 2,  18, 24, 16, 30, 6, 26, 20, 10, 1,
  8, 14, 25, 3,  4,  29, 11, 19, 32, 12, 22, 7, 5,  27, 15, 21
];

const S_BOX : [[u8; 64]; 8] = [
  [
    14, 4,  13, 1, 2,  15, 11, 8,  3,  10, 6,  12, 5,  9,  0, 7,
    0,  15, 7,  4, 14, 2,  13, 1,  10, 6,  12, 11, 9,  5,  3, 8,
    4,  1,  14, 8, 13, 6,  2,  11, 15, 12, 9,  7,  3,  10, 5, 0,
    15, 12, 8,  2, 4,  9,  1,  7,  5,  11, 3,  14, 10, 0,  6, 13
  ],
  [
    15, 1,  8,  14, 6,  11, 3,  4,  9,  7,  2,  13, 12, 0,  5,  10,
    3,  13, 4,  7,  15, 2,  8,  14, 12, 0,  1,  10, 6,  9,  11, 5,
    0,  14, 7,  11, 10, 4,  13, 1,  5,  8,  12, 6,  9,  3,  2,  15,
    13, 8,  10, 1,  3,  15, 4,  2,  11, 6,  7,  12, 5,  9,  0,  14 1,
    2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15, 16
  ]
];

fn generate_subkeys(key
                    : u16)
    ->[u8; 6] {
      let mut subkeys = [0; 6];
      let mut rng = StdRng::seed_from_u64(key as u64);
for
  i in 0..6 { subkeys[i] = rng.gen(); }

subkeys
    }

fn apply_permutation_table(block
                           : u32)
    ->u32 {
  let mut result = 0;
for
  i in 0..64 {
    let block_bit = block & (1 << (31 - PERMUTATION_TABLE[i]));
    if block_bit
      != 0 { result |= 1 << (63 - i); }
  }
result
}

fn apply_inverse_permutation_table(block : u64)->u32 {
  let mut result = 0;
for
  i in 0..64 {
    let block_bit = block & (1 << (63 - INVERSE_PERMUTATION_TABLE[i]));
    if block_bit
      != 0 { result |= 1 << (31 - i); }
  }
result
}

fn apply_s_box(input : u8, s_box_index : usize)->u8 {
  let row = ((input & 0b100000) >> 4) | (input & 0b000001);
  let col = (input & 0b011110) >> 1;
  S_BOX[s_box_index][row as usize * 16 + col as usize]
}

fn round(block : u32, subkey : u8)->u32 {
  // Split the block into 4 bytes
  let mut bytes = [0u8; 4];
for
  i in 0..4 { bytes[i] = ((block >> ((3 - i) * 8)) & 0xff) as u8; }
// Apply the substitution to each byte
bytes[0] = apply_s_box(bytes[0] ^ subkey, 0);
bytes[1] = apply_s_box(bytes[1], 1);
bytes[2] = apply_s_box(bytes[2], 2);
bytes[3] = apply_s_box(bytes[3], 3);

// Combine the bytes into a single 32-bit block
let mut result = 0;
for
  i in 0..4 { result |= (bytes[i] as u32) << ((3 - i) * 8); }

// Apply the permutation to the block
apply_permutation_table(result)
}

fn des_block(block : u64, key : u16, decrypt : bool)->u64 {
  let subkeys = generate_subkeys(key);
  let mut left_half = ((block >> 32) & 0xffffffff) as u32;
  let mut right_half = (block & 0xffffffff) as u32;

  let mut round_function = | block : u32, subkey : u8 |->u32 {
    round(
        block, if decrypt {
          subkeys[5 - subkey as usize]
        } else {subkeys[subkey as usize]})
  };

  if decrypt {
    for
      i in(0..8).rev() {
        let temp = left_half;
        left_half = right_half;
        right_half = temp ^ round_function(right_half, i);
      }
  } else {
for
  i in 0..8 {
    let temp = right_half;
    right_half = left_half;
    left_half = temp ^ round_function(left_half, i);
  }
  }

  // The final block is the concatenation of the swapped left and right halves,
  // with the inverse permutation applied
  let mut final_block = ((right_half as u64) << 32) | (left_half as u64);
  final_block = permute(final_block, &INVERSE_PERMUTATION_TABLE);

  final_block
}
