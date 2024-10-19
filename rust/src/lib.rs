mod utils;

use utils::MD5Builder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MD5 {
    buffer: Vec<u8>,
    builder: MD5Builder,
}

impl Default for MD5 {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl MD5 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MD5 {
        MD5 {
            buffer: Vec::new(),
            builder: MD5Builder::new(),
        }
    }

    pub fn digest(&self) -> Option<String> {
        self.builder.digest.clone()
    }

    fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    pub fn update(&mut self, val: &[u8]) {
        self.buffer.extend_from_slice(val);
        if self.buffer_size() <= 64 {
            return;
        }
        let mut chunks = self.buffer.chunks_exact(64);
        for chunk in &mut chunks {
            self.builder.update(chunk.to_vec(), false);
        }
        let last_block = chunks.remainder().to_vec();
        self.buffer.clear();
        self.buffer.extend_from_slice(&last_block);
    }

    pub fn finalize(&mut self) {
        if self.digest().is_none() {
            self.builder.update(self.buffer.to_vec(), true);
        }
    }
}

#[wasm_bindgen]
pub fn md5_from_array_buffer(array_buffer: &js_sys::ArrayBuffer) -> String {
    let byte_array = js_sys::Uint8Array::new(&array_buffer);
    let mut md5 = MD5::new();
    md5.update(byte_array.to_vec().as_slice());
    md5.finalize();
    md5.digest().unwrap()
}

#[wasm_bindgen]
pub fn md5_from_string(val: js_sys::JsString) -> String {
    let mut md5 = MD5::new();
    md5.update(val.as_string().unwrap().as_bytes());
    md5.finalize();
    md5.digest().unwrap()
}

#[wasm_bindgen]
pub async fn md5_from_blob(blob: &web_sys::Blob) -> String {
    const CHUNK_SIZE: f64 = 100.0 * 1024.0 * 1024.0; // 100MB
    let mut md5 = MD5::new();

    // If the file is smaller than the chunk size, we can read it all at once
    if blob.size() < CHUNK_SIZE {
        let blob = blob.slice_with_f64_and_f64(0.0, blob.size()).unwrap();
        let array_buffer = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
            .await
            .unwrap();
        let byte_array = js_sys::Uint8Array::new(&array_buffer);
        md5.update(byte_array.to_vec().as_slice());
        md5.finalize();
        return md5.digest().unwrap();
    }

    // Otherwise, we need to read the file in chunks
    let mut start = f64::from(0);
    while start < blob.size() {
        let end = (start + CHUNK_SIZE).min(blob.size());
        let blob = blob.slice_with_f64_and_f64(start, end).unwrap();
        let array_buffer = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
            .await
            .unwrap();
        let byte_array = js_sys::Uint8Array::new(&array_buffer);
        md5.update(byte_array.to_vec().as_slice());
        start = end;
    }
    md5.finalize();
    md5.digest().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let mut md5 = MD5::new();
        md5.update("".as_bytes());
        md5.finalize();
        assert!(md5.digest() == Some("d41d8cd98f00b204e9800998ecf8427e".to_string()));
    }
    #[test]
    fn a() {
        let mut md5 = MD5::new();
        md5.update("a".as_bytes());
        md5.finalize();
        assert!(md5.digest() == Some("0cc175b9c0f1b6a831c399e269772661".to_string()));
    }
    #[test]
    fn abc() {
        let mut md5 = MD5::new();
        md5.update("abc".as_bytes());
        md5.finalize();
        assert!(md5.digest() == Some("900150983cd24fb0d6963f7d28e17f72".to_string()));
    }
    #[test]
    fn short_string1() {
        let mut md5 = MD5::new();
        md5.update("message digest".as_bytes());
        md5.finalize();
        assert!(md5.digest() == Some("f96b697d7cb7938d525a2f31aaf161d0".to_string()));
    }
    #[test]
    fn short_string2() {
        let mut md5 = MD5::new();
        md5.update("abcdefghijklmnopqrstuvwxyz".as_bytes());
        md5.finalize();
        assert!(md5.digest() == Some("c3fcd3d76192e4007dfb496cca67e13b".to_string()));
    }
    #[test]
    fn long_string1() {
        let mut md5 = MD5::new();
        md5.update("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes());
        md5.finalize();
        assert!(md5.digest() == Some("d174ab98d277d9f5a5611c2c9f419d9f".to_string()));

        let mut md5 = MD5::new();
        md5.update("ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes());
        md5.update("abcdefghijklmnopqrstuvwxyz0123456789".as_bytes());
        md5.finalize();
        assert!(md5.digest() == Some("d174ab98d277d9f5a5611c2c9f419d9f".to_string()));
    }
    #[test]
    fn long_string2() {
        let mut md5 = MD5::new();
        md5.update(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                .as_bytes(),
        );
        md5.finalize();
        assert!(md5.digest() == Some("57edf4a22be3c955ac49da2e2107b67a".to_string()));

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
        assert!(md5.digest() == Some("57edf4a22be3c955ac49da2e2107b67a".to_string()));
    }
}
