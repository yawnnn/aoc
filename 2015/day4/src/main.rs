use std::fmt;

struct Round(u8);

impl Round {
    const NUM_ROUNDS: u8 = 64;

    #[rustfmt::skip]
    const SHIFT: [u8; Self::NUM_ROUNDS as usize] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5, 9 , 14, 20, 5, 9 , 14, 20, 5, 9 , 14, 20, 5, 9 , 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    #[rustfmt::skip]
    const K: [u32; Self::NUM_ROUNDS as usize] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
    ];

    fn rounds() -> impl Iterator<Item = Round> {
        (0..Self::NUM_ROUNDS).map(Round)
    }

    fn fx(&self, bb: u32, cc: u32, dd: u32) -> u32 {
        match self.0 {
            0..16 => (bb & cc) | ((!bb) & dd),
            16..32 => (dd & bb) | ((!dd) & cc),
            32..48 => bb ^ cc ^ dd,
            48..64 => cc ^ (bb | (!dd)),
            _ => unreachable!(),
        }
    }

    fn k(&self) -> u32 {
        Self::K[self.0 as usize]
    }

    fn shift(&self) -> u32 {
        Self::SHIFT[self.0 as usize] as u32
    }

    fn input(&self, block: &[u32; 16]) -> u32 {
        let index = match self.0 {
            0..16 => self.0 as u32,
            16..32 => (self.0 as u32 * 5 + 1) % 16,
            32..48 => (self.0 as u32 * 3 + 5) % 16,
            48..64 => (self.0 as u32 * 7) % 16,
            _ => unreachable!(),
        };

        block[index as usize]
    }

    fn transform(&self, block: &[u32; 16], state: &mut [u32; 4]) {
        let (aa, bb, cc, dd) = (state[0], state[1], state[2], state[3]);

        let mut tmp = aa
            .wrapping_add(self.fx(bb, cc, dd))
            .wrapping_add(self.input(block))
            .wrapping_add(self.k());
        tmp = tmp.rotate_left(self.shift());
        tmp = tmp.wrapping_add(bb);

        *state = [dd, tmp, bb, cc];
    }
}

fn digest_block(input: &[u8; 64], digest: &mut [u32; 4]) {
    let mut block = [0u32; 16];

    input
        .chunks_exact(4)
        .zip(&mut block)
        .for_each(|(chunk, b)| *b = u32::from_ne_bytes(chunk.try_into().unwrap()).to_le());

    let mut state = *digest;

    for round in Round::rounds() {
        round.transform(&block, &mut state);
    }

    digest.iter_mut().zip(&state).for_each(|(d, &s)| {
        *d = d.wrapping_add(s);
    });
}

fn md5<T: AsRef<[u8]>>(data: T) -> [u8; 16] {
    let input = data.as_ref();

    let a0 = 0x67452301;
    let b0 = 0xefcdab89;
    let c0 = 0x98badcfe;
    let d0 = 0x10325476;
    let mut state = [a0, b0, c0, d0];

    let mut block = [0u8; 64];
    let block_size = std::mem::size_of_val(&block);
    let mut read = 0;

    // digest message in 64 byte blocks
    while input.len() - read >= block_size {
        block.copy_from_slice(&input[read..read + block_size]);
        digest_block(&block, &mut state);
        read += block_size;
    }

    // last iteration w/ padding
    block = [0u8; 64];
    let mut pos = 0;

    let remaining = &input[read..];
    block[pos..remaining.len()].copy_from_slice(remaining);
    pos = remaining.len();

    block[pos] = 0x80;
    pos += 1;
    block[pos..].fill(0);

    let input_len = input.len() as u64 * 8;
    block[56..64].copy_from_slice(&input_len.to_le_bytes());

    digest_block(&block, &mut state);

    let mut digest = [0; 16];

    digest
        .iter_mut()
        .zip(state.iter().flat_map(|&word| u32::from_le(word).to_ne_bytes()))
        .for_each(|(out, byte)| *out = byte);

    digest
}

struct Digest([u8; 16]);

impl fmt::LowerHex for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.0 {
            write!(f, "{v:02x}")?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> Option<u64> {
    for n in 0..=u64::MAX {
        let data = format!("{}{}", input, n);
        let digest = md5(data);

        //if digest[0..2] == [0; 2] && digest[2] < 0x10 {
        if format!("{:02x}", Digest(digest)).starts_with("00000") {
            return Some(n);
        }
    }
    None
}

fn part2(input: &str) -> Option<u64> {
    for n in 0..=u64::MAX {
        let data = format!("{}{}", input, n);
        let digest = md5(data);

        if digest[0..3] == [0; 3] {
        //if format!("{:x}", Digest(digest)).starts_with("000000") {
            return Some(n);
        }
    }
    None
}

fn main() {
    let input = "yzbqklnj";
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
