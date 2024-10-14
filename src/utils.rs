use std::{convert::TryInto, fmt};

pub struct Buffer {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x}{:08x}{:08x}{:08x}",
            self.a.swap_bytes(),
            self.b.swap_bytes(),
            self.c.swap_bytes(),
            self.d.swap_bytes()
        )
    }
}

pub struct MD5Builder {
    pub state: Buffer,
    total_length: u64,
    pub digest: Option<String>,
}

impl MD5Builder {
    fn padding(&self, value: &mut Vec<u8>) {
        // Step 1: Append Padding Bits
        value.push(0b10000000); // Append "1" bit
        while (value.len() * 8) % 512 != 448 {
            // 448 = 512 - 64
            value.push(0u8); // Append "0" bits
        }
        /*
        Step 2. Append Length (64 bits integer)
        A 64-bit representation of b is appended to the result of the previous step
        */
        value.extend(&[
            self.total_length as u8,
            (self.total_length >> 8) as u8,
            (self.total_length >> 16) as u8,
            (self.total_length >> 24) as u8,
            (self.total_length >> 32) as u8,
            (self.total_length >> 40) as u8,
            (self.total_length >> 48) as u8,
            (self.total_length >> 56) as u8,
        ]);
    }

    pub fn update(&mut self, mut value: Vec<u8>, padding: bool) {
        self.total_length = self
            .total_length
            .wrapping_add(value.len().saturating_mul(8) as u64);
        if padding {
            self.padding(&mut value);
        }

        let (mut a, mut b, mut c, mut d) = (self.state.a, self.state.b, self.state.c, self.state.d);
        for block in value.chunks_exact_mut(64) {
            let mut x = [0u32; 16];
            for (j, chunk) in block.chunks_exact(4).enumerate() {
                x[j] = u32::from_le_bytes(chunk.try_into().unwrap());
            }

            // Save initial values of A, B, C, D
            let (aa, bb, cc, dd) = (a, b, c, d);

            let round1 = |a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, t: u32| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(b & c | !b & d)
                        .wrapping_add(x)
                        .wrapping_add(t)
                        .rotate_left(s),
                )
            };

            // Perform Round 1 operations
            round1(&mut a, b, c, d, x[0], 7, 0xd76aa478);
            round1(&mut d, a, b, c, x[1], 12, 0xe8c7b756);
            round1(&mut c, d, a, b, x[2], 17, 0x242070db);
            round1(&mut b, c, d, a, x[3], 22, 0xc1bdceee);

            round1(&mut a, b, c, d, x[4], 7, 0xf57c0faf);
            round1(&mut d, a, b, c, x[5], 12, 0x4787c62a);
            round1(&mut c, d, a, b, x[6], 17, 0xa8304613);
            round1(&mut b, c, d, a, x[7], 22, 0xfd469501);

            round1(&mut a, b, c, d, x[8], 7, 0x698098d8);
            round1(&mut d, a, b, c, x[9], 12, 0x8b44f7af);
            round1(&mut c, d, a, b, x[10], 17, 0xffff5bb1);
            round1(&mut b, c, d, a, x[11], 22, 0x895cd7be);

            round1(&mut a, b, c, d, x[12], 7, 0x6b901122);
            round1(&mut d, a, b, c, x[13], 12, 0xfd987193);
            round1(&mut c, d, a, b, x[14], 17, 0xa679438e);
            round1(&mut b, c, d, a, x[15], 22, 0x49b40821);

            // Closure for Round 2 operations
            let round2 = |a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, t: u32| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(b & d | c & !d)
                        .wrapping_add(x)
                        .wrapping_add(t)
                        .rotate_left(s),
                )
            };

            // Perform Round 2 operations
            round2(&mut a, b, c, d, x[1], 5, 0xf61e2562);
            round2(&mut d, a, b, c, x[6], 9, 0xc040b340);
            round2(&mut c, d, a, b, x[11], 14, 0x265e5a51);
            round2(&mut b, c, d, a, x[0], 20, 0xe9b6c7aa);
            round2(&mut a, b, c, d, x[5], 5, 0xd62f105d);
            round2(&mut d, a, b, c, x[10], 9, 0x02441453);
            round2(&mut c, d, a, b, x[15], 14, 0xd8a1e681);
            round2(&mut b, c, d, a, x[4], 20, 0xe7d3fbc8);
            round2(&mut a, b, c, d, x[9], 5, 0x21e1cde6);
            round2(&mut d, a, b, c, x[14], 9, 0xc33707d6);
            round2(&mut c, d, a, b, x[3], 14, 0xf4d50d87);
            round2(&mut b, c, d, a, x[8], 20, 0x455a14ed);
            round2(&mut a, b, c, d, x[13], 5, 0xa9e3e905);
            round2(&mut d, a, b, c, x[2], 9, 0xfcefa3f8);
            round2(&mut c, d, a, b, x[7], 14, 0x676f02d9);
            round2(&mut b, c, d, a, x[12], 20, 0x8d2a4c8a);

            // Closure for Round 3 operations
            let round3 = |a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, t: u32| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(b ^ c ^ d)
                        .wrapping_add(x)
                        .wrapping_add(t)
                        .rotate_left(s),
                )
            };

            // Perform Round 3 operations
            round3(&mut a, b, c, d, x[5], 4, 0xfffa3942);
            round3(&mut d, a, b, c, x[8], 11, 0x8771f681);
            round3(&mut c, d, a, b, x[11], 16, 0x6d9d6122);
            round3(&mut b, c, d, a, x[14], 23, 0xfde5380c);
            round3(&mut a, b, c, d, x[1], 4, 0xa4beea44);
            round3(&mut d, a, b, c, x[4], 11, 0x4bdecfa9);
            round3(&mut c, d, a, b, x[7], 16, 0xf6bb4b60);
            round3(&mut b, c, d, a, x[10], 23, 0xbebfbc70);
            round3(&mut a, b, c, d, x[13], 4, 0x289b7ec6);
            round3(&mut d, a, b, c, x[0], 11, 0xeaa127fa);
            round3(&mut c, d, a, b, x[3], 16, 0xd4ef3085);
            round3(&mut b, c, d, a, x[6], 23, 0x04881d05);
            round3(&mut a, b, c, d, x[9], 4, 0xd9d4d039);
            round3(&mut d, a, b, c, x[12], 11, 0xe6db99e5);
            round3(&mut c, d, a, b, x[15], 16, 0x1fa27cf8);
            round3(&mut b, c, d, a, x[2], 23, 0xc4ac5665);

            // Closure for Round 4 operations
            let round4 = |a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, t: u32| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(c ^ (b | !d))
                        .wrapping_add(x)
                        .wrapping_add(t)
                        .rotate_left(s),
                )
            };

            round4(&mut a, b, c, d, x[0], 6, 0xf4292244);
            round4(&mut d, a, b, c, x[7], 10, 0x432aff97);
            round4(&mut c, d, a, b, x[14], 15, 0xab9423a7);
            round4(&mut b, c, d, a, x[5], 21, 0xfc93a039);
            round4(&mut a, b, c, d, x[12], 6, 0x655b59c3);
            round4(&mut d, a, b, c, x[3], 10, 0x8f0ccc92);
            round4(&mut c, d, a, b, x[10], 15, 0xffeff47d);
            round4(&mut b, c, d, a, x[1], 21, 0x85845dd1);
            round4(&mut a, b, c, d, x[8], 6, 0x6fa87e4f);
            round4(&mut d, a, b, c, x[15], 10, 0xfe2ce6e0);
            round4(&mut c, d, a, b, x[6], 15, 0xa3014314);
            round4(&mut b, c, d, a, x[13], 21, 0x4e0811a1);
            round4(&mut a, b, c, d, x[4], 6, 0xf7537e82);
            round4(&mut d, a, b, c, x[11], 10, 0xbd3af235);
            round4(&mut c, d, a, b, x[2], 15, 0x2ad7d2bb);
            round4(&mut b, c, d, a, x[9], 21, 0xeb86d391);

            a = a.wrapping_add(aa);
            b = b.wrapping_add(bb);
            c = c.wrapping_add(cc);
            d = d.wrapping_add(dd);
        }
        self.state.a = a;
        self.state.b = b;
        self.state.c = c;
        self.state.d = d;

        if padding {
            self.digest = Some(self.state.to_string());
        }
    }

    pub fn new() -> Self {
        Self {
            state: Buffer {
                a: u32::from_le_bytes([0x01, 0x23, 0x45, 0x67]),
                b: u32::from_le_bytes([0x89, 0xab, 0xcd, 0xef]),
                c: u32::from_le_bytes([0xfe, 0xdc, 0xba, 0x98]),
                d: u32::from_le_bytes([0x76, 0x54, 0x32, 0x10]),
            },
            total_length: 0,
            digest: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        fn hash_from_string(value: &str) -> String {
            let mut builder = MD5Builder::new();
            if value.is_empty() {
                builder.update(value.as_bytes().to_vec(), true);
                return builder.state.to_string();
            }
            for block in value.as_bytes().to_vec().chunks(64) {
                match block.len() == 64 {
                    true => {
                        builder.update(block.to_vec(), false);
                    }
                    false => {
                        builder.update(block.to_vec(), true);
                    }
                }
            }
            builder.state.to_string()
        }
        assert!(hash_from_string("") == "d41d8cd98f00b204e9800998ecf8427e");
        assert!(hash_from_string("a") == "0cc175b9c0f1b6a831c399e269772661");
        assert!(hash_from_string("abc") == "900150983cd24fb0d6963f7d28e17f72");
        assert!(hash_from_string("message digest") == "f96b697d7cb7938d525a2f31aaf161d0");
        assert!(
            hash_from_string("abcdefghijklmnopqrstuvwxyz") == "c3fcd3d76192e4007dfb496cca67e13b"
        );
        assert!(
            hash_from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
                == "d174ab98d277d9f5a5611c2c9f419d9f"
        );
        assert!(
            hash_from_string(
                "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
            ) == "57edf4a22be3c955ac49da2e2107b67a"
        );
    }
}
