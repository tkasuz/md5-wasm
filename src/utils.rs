use std::convert::TryInto;

const fn F(x: u32, y: u32, z: u32) -> u32 {
    x & y | !x & z
}
const fn G(x: u32, y: u32, z: u32) -> u32 {
    x & z | y & !z
}
const fn H(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}
const fn I(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

const T: [u32; 65] = [
    0x00000000, 0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
    0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e,
    0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681,
    0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9,
    0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60,
    0xbebfbc70, 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8,
    0xc4ac5665, 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d,
    0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
    0xeb86d391,
];

#[derive(Clone, Copy)]
pub struct Buffer {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Buffer {
    fn to_string(self) -> String {
        format!(
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

            let round1 = |a: &mut u32, b: u32, c: u32, d: u32, k: usize, s: u32, i: usize| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(F(b, c, d))
                        .wrapping_add(x[k])
                        .wrapping_add(T[i])
                        .rotate_left(s),
                )
            };

            // Perform Round 1 operations
            round1(&mut a, b, c, d, 0, 7, 1);
            round1(&mut d, a, b, c, 1, 12, 2);
            round1(&mut c, d, a, b, 2, 17, 3);
            round1(&mut b, c, d, a, 3, 22, 4);

            round1(&mut a, b, c, d, 4, 7, 5);
            round1(&mut d, a, b, c, 5, 12, 6);
            round1(&mut c, d, a, b, 6, 17, 7);
            round1(&mut b, c, d, a, 7, 22, 8);

            round1(&mut a, b, c, d, 8, 7, 9);
            round1(&mut d, a, b, c, 9, 12, 10);
            round1(&mut c, d, a, b, 10, 17, 11);
            round1(&mut b, c, d, a, 11, 22, 12);

            round1(&mut a, b, c, d, 12, 7, 13);
            round1(&mut d, a, b, c, 13, 12, 14);
            round1(&mut c, d, a, b, 14, 17, 15);
            round1(&mut b, c, d, a, 15, 22, 16);

            // Closure for Round 2 operations
            let round2 = |a: &mut u32, b: u32, c: u32, d: u32, k: usize, s: u32, i: usize| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(G(b, c, d))
                        .wrapping_add(x[k])
                        .wrapping_add(T[i])
                        .rotate_left(s),
                )
            };

            // Perform Round 2 operations
            round2(&mut a, b, c, d, 1, 5, 17);
            round2(&mut d, a, b, c, 6, 9, 18);
            round2(&mut c, d, a, b, 11, 14, 19);
            round2(&mut b, c, d, a, 0, 20, 20);
            round2(&mut a, b, c, d, 5, 5, 21);
            round2(&mut d, a, b, c, 10, 9, 22);
            round2(&mut c, d, a, b, 15, 14, 23);
            round2(&mut b, c, d, a, 4, 20, 24);
            round2(&mut a, b, c, d, 9, 5, 25);
            round2(&mut d, a, b, c, 14, 9, 26);
            round2(&mut c, d, a, b, 3, 14, 27);
            round2(&mut b, c, d, a, 8, 20, 28);
            round2(&mut a, b, c, d, 13, 5, 29);
            round2(&mut d, a, b, c, 2, 9, 30);
            round2(&mut c, d, a, b, 7, 14, 31);
            round2(&mut b, c, d, a, 12, 20, 32);

            // Closure for Round 3 operations
            let round3 = |a: &mut u32, b: u32, c: u32, d: u32, k: usize, s: u32, i: usize| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(H(b, c, d))
                        .wrapping_add(x[k])
                        .wrapping_add(T[i])
                        .rotate_left(s),
                )
            };

            // Perform Round 3 operations
            round3(&mut a, b, c, d, 5, 4, 33);
            round3(&mut d, a, b, c, 8, 11, 34);
            round3(&mut c, d, a, b, 11, 16, 35);
            round3(&mut b, c, d, a, 14, 23, 36);
            round3(&mut a, b, c, d, 1, 4, 37);
            round3(&mut d, a, b, c, 4, 11, 38);
            round3(&mut c, d, a, b, 7, 16, 39);
            round3(&mut b, c, d, a, 10, 23, 40);
            round3(&mut a, b, c, d, 13, 4, 41);
            round3(&mut d, a, b, c, 0, 11, 42);
            round3(&mut c, d, a, b, 3, 16, 43);
            round3(&mut b, c, d, a, 6, 23, 44);
            round3(&mut a, b, c, d, 9, 4, 45);
            round3(&mut d, a, b, c, 12, 11, 46);
            round3(&mut c, d, a, b, 15, 16, 47);
            round3(&mut b, c, d, a, 2, 23, 48);

            // Closure for Round 4 operations
            let round4 = |a: &mut u32, b: u32, c: u32, d: u32, k: usize, s: u32, i: usize| {
                *a = b.wrapping_add(
                    (*a).wrapping_add(I(b, c, d))
                        .wrapping_add(x[k])
                        .wrapping_add(T[i])
                        .rotate_left(s),
                )
            };

            round4(&mut a, b, c, d, 0, 6, 49);
            round4(&mut d, a, b, c, 7, 10, 50);
            round4(&mut c, d, a, b, 14, 15, 51);
            round4(&mut b, c, d, a, 5, 21, 52);
            round4(&mut a, b, c, d, 12, 6, 53);
            round4(&mut d, a, b, c, 3, 10, 54);
            round4(&mut c, d, a, b, 10, 15, 55);
            round4(&mut b, c, d, a, 1, 21, 56);
            round4(&mut a, b, c, d, 8, 6, 57);
            round4(&mut d, a, b, c, 15, 10, 58);
            round4(&mut c, d, a, b, 6, 15, 59);
            round4(&mut b, c, d, a, 13, 21, 60);
            round4(&mut a, b, c, d, 4, 6, 61);
            round4(&mut d, a, b, c, 11, 10, 62);
            round4(&mut c, d, a, b, 2, 15, 63);
            round4(&mut b, c, d, a, 9, 21, 64);

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
            if value.len() == 0 {
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
