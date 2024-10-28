mod utils;

use utils::{padding, Status};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Md5 {
    status: Status,
    size: u64,
    buffer: Vec<u8>,
}

impl Default for Md5 {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Md5 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            status: Status::default(),
            size: 0,
            buffer: Vec::new(),
        }
    }

    pub fn update(&mut self, val: &[u8]) {
        self.size = self.size.wrapping_add(val.len() as u64);
        self.buffer.extend_from_slice(val);
        let mut iter = self.buffer.chunks_exact(64);
        for chunk in iter.by_ref() {
            self.status.update(chunk);
        }
        self.buffer = iter.remainder().to_vec();
    }

    pub fn finalize(&mut self) {
        let mut v = Vec::<u8>::with_capacity(self.buffer.len() + 521);
        v.append(self.buffer.to_vec().as_mut());
        padding(self.size, &mut v);
        self.status.update(v.as_slice());
    }

    pub fn digest(&self) -> String {
        self.status.digest()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let input = "".as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        print!("hex: {}", hex);
        assert!(hex == "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn a() {
        let input = "a".as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        print!("hex: {}", hex);
        assert!(hex == "0cc175b9c0f1b6a831c399e269772661");
    }
    #[test]
    fn abc() {
        let input = "abc".as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "900150983cd24fb0d6963f7d28e17f72");
    }
    #[test]
    fn short_string1() {
        let input = "message digest".as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "f96b697d7cb7938d525a2f31aaf161d0");
    }
    #[test]
    fn short_string2() {
        let input = "abcdefghijklmnopqrstuvwxyz".as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "c3fcd3d76192e4007dfb496cca67e13b");
    }
    #[test]
    fn long_string1() {
        let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "d174ab98d277d9f5a5611c2c9f419d9f");
    }
    #[test]
    fn long_string2() {
        let input =
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                .as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "57edf4a22be3c955ac49da2e2107b67a");
    }

    #[test]
    fn boundary_64_bytes() {
        let input = "a".repeat(64);
        let input = input.as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "014842d480b571495a4a0363793f7367");
    }

    #[test]
    fn boundary_65_bytes() {
        let input = "a".repeat(65);
        let input = input.as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "c743a45e0d2e6a95cb859adae0248435");
    }

    #[test]
    fn boundary_128_bytes() {
        let input = "a".repeat(128);
        let input = input.as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "e510683b3f5ffe4093d021808bc6ff70");
    }

    #[test]
    fn boundary_129_bytes() {
        let input = "a".repeat(129);
        let input = input.as_bytes();
        let mut status = Md5::new();
        status.update(input);
        status.finalize();
        let hex = status.digest();
        assert!(hex == "b325dc1c6f5e7a2b7cf465b9feab7948");
    }
}
