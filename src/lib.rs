mod utils;

use utils::MD5Builder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub struct MD5  {
    buffer: Vec<u8>,
    builder: MD5Builder,
}

#[wasm_bindgen]
impl MD5 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MD5 {
        MD5 { buffer: Vec::new(), builder: MD5Builder::new() }
    }

    pub fn digest(&self) -> String {
        self.builder.state.to_string()
    }

    pub fn update(&mut self, val: &[u8]) {
        self.buffer.extend_from_slice(val);
        if self.buffer.len() <= 64 {
            return;
        }
        let buffer = self.buffer.clone();
        for block in buffer.chunks(64) {
            match block.len() {
                64 => self.builder.update(block.to_vec(), false),
                _ => {
                    self.buffer.clear();
                    self.buffer.extend_from_slice(block);
                }
            }
        }
    }
    
    pub fn finalize(&mut self) {
        self.builder.update(self.buffer.clone(), true);
    }    
}

#[wasm_bindgen]
pub async fn from_file(file: &web_sys::File) -> String {
    let mut md5 = MD5::new();
    let bits_size = 8 * file.size().ceil() as i32;
    
    let mut start = 0;
    let mut end = start + 8;
    while end < bits_size {
        let blob = match end > file.size() as i32 {
            true => {
                file.slice_with_i32_and_f64(start, file.size()).unwrap()
            },
            false => {
                file.slice_with_i32_and_i32(start, end).unwrap()
            },
        };
        let array_buffer = wasm_bindgen_futures::JsFuture::from(blob.array_buffer()).await.unwrap();
        let byte_array = js_sys::Uint8Array::new(&array_buffer);
        md5.update(byte_array.to_vec().as_slice());
        start = end;
        end = end + 8;
    };
    md5.finalize();
    md5.digest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let mut md5 = MD5::new();
        md5.update("".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn a() {
        let mut md5 = MD5::new();
        md5.update("a".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "0cc175b9c0f1b6a831c399e269772661");
    }
    #[test]
    fn abc() {
        let mut md5 = MD5::new();
        md5.update("abc".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "900150983cd24fb0d6963f7d28e17f72");
    }
    #[test]
    fn short_string1() {
        let mut md5 = MD5::new();
        md5.update("message digest".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "f96b697d7cb7938d525a2f31aaf161d0");
    }
    #[test]
    fn short_string2() {
        let mut md5 = MD5::new();
        md5.update("abcdefghijklmnopqrstuvwxyz".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "c3fcd3d76192e4007dfb496cca67e13b");
    }
    #[test]
    fn long_string1() {
        let mut md5 = MD5::new();
        md5.update("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "d174ab98d277d9f5a5611c2c9f419d9f");
        
        let mut md5 = MD5::new();
        md5.update("ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes());
        md5.update("abcdefghijklmnopqrstuvwxyz0123456789".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "d174ab98d277d9f5a5611c2c9f419d9f");
    }
    #[test]
    fn long_string2() {
        let mut md5 = MD5::new();
        md5.update("12345678901234567890123456789012345678901234567890123456789012345678901234567890".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "57edf4a22be3c955ac49da2e2107b67a");

        let mut md5 = MD5::new();
        md5.update("1234567890".as_bytes());
        md5.update("1234567890".as_bytes());
        md5.update("1234567890".as_bytes());
        md5.update("1234567890".as_bytes());
        md5.update("1234567890".as_bytes());
        md5.update("1234567890".as_bytes());
        md5.update("1234567890".as_bytes());
        md5.update("1234567890".as_bytes());
        md5.finalize();
        assert!(md5.digest() == "57edf4a22be3c955ac49da2e2107b67a");
    }
}