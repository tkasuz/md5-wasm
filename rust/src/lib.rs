mod utils;

use utils::{padding, Status};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn md5_from_u8_array(val: &[u8]) -> String {
    const CHUNK_SIZE: usize = 200 * 1024 * 1024; // 100 MB
    let mut status = Status::default();
    let size = val.len();
    if size == 0 {
        let mut val_vec = val.to_vec();
        let final_chunk = padding(size as u64, &mut val_vec);
        status.update(final_chunk);
        return status.digest();
    }

    for chunk in val.chunks(CHUNK_SIZE) {
        if chunk.len() < CHUNK_SIZE {
            let mut chunk_vec = chunk.to_vec();
            let final_chunk = padding(size as u64, &mut chunk_vec);
            status.update(final_chunk);
        } else {
            status.update(chunk);
        }
    }
    status.digest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let hex = md5_from_u8_array("".as_bytes());
        print!("hex: {}", hex);
        assert!(hex == "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn a() {
        let hex = md5_from_u8_array("a".as_bytes());
        assert!(hex == "0cc175b9c0f1b6a831c399e269772661");
    }
    #[test]
    fn abc() {
        let hex = md5_from_u8_array("abc".as_bytes());
        assert!(hex == "900150983cd24fb0d6963f7d28e17f72");
    }
    #[test]
    fn short_string1() {
        let hex = md5_from_u8_array("message digest".as_bytes());
        assert!(hex == "f96b697d7cb7938d525a2f31aaf161d0");
    }
    #[test]
    fn short_string2() {
        let hex = md5_from_u8_array("abcdefghijklmnopqrstuvwxyz".as_bytes());
        assert!(hex == "c3fcd3d76192e4007dfb496cca67e13b");
    }
    #[test]
    fn long_string1() {
        let hex = md5_from_u8_array(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes(),
        );
        assert!(hex == "d174ab98d277d9f5a5611c2c9f419d9f");
    }
    #[test]
    fn long_string2() {
        let hex = md5_from_u8_array(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                .as_bytes(),
        );
        assert!(hex == "57edf4a22be3c955ac49da2e2107b67a");
    }
}
