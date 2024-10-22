mod utils;

use utils::{padding, Status};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn md5_from_array_buffer(val: &js_sys::ArrayBuffer) -> String {
    let status = Status::default();
    let length = val.byte_length();
    let val = js_sys::Uint8Array::new(val);
    let mut val = val.to_vec();
    let val = padding(length as u64, val.as_mut());
    let status = status.update(val);
    status.digest()
}

fn from_string(val: String) -> String {
    let status = Status::default();
    let length = val.len();
    let mut val = val.as_bytes().to_vec();
    let val = padding(length as u64, val.as_mut());
    let status = status.update(val);
    status.digest()
}

#[wasm_bindgen]
pub fn md5_from_string(val: js_sys::JsString) -> String {
    let val = val.as_string().unwrap();
    from_string(val)
}

#[wasm_bindgen]
pub async fn md5_from_file(file: &web_sys::File) -> String {
    const CHUNK_SIZE: f64 = 100.0 * 1024.0 * 1024.0; // 100 MB
    let mut status = Status::default();
    let file_size = file.size();

    let mut start = 0.0;
    let mut end = start + CHUNK_SIZE;
    let mut is_final_chunk = false;

    loop {
        let blob_slice = match end >= file_size {
            true => {
                is_final_chunk = true;
                file.slice_with_f64(start).unwrap()
            }
            false => file.slice_with_f64_and_f64(start, end).unwrap(),
        };
        let array_buffer = wasm_bindgen_futures::JsFuture::from(blob_slice.array_buffer())
            .await
            .unwrap();
        let byte_array = js_sys::Uint8Array::new(&array_buffer);
        let mut value = byte_array.to_vec();
        if is_final_chunk {
            let padding_value = padding(file_size as u64, value.as_mut());
            status = status.update(padding_value);
            break;
        } else {
            status = status.update(value.as_slice());
        }
        start = end;
        end = start + CHUNK_SIZE;
    }
    status.digest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let hex = from_string("".to_string());
        assert!(hex == "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn a() {
        let hex = from_string("a".to_string());
        assert!(hex == "0cc175b9c0f1b6a831c399e269772661");
    }
    #[test]
    fn abc() {
        let hex = from_string("abc".to_string());
        assert!(hex == "900150983cd24fb0d6963f7d28e17f72");
    }
    #[test]
    fn short_string1() {
        let hex = from_string("message digest".to_string());
        assert!(hex == "f96b697d7cb7938d525a2f31aaf161d0");
    }
    #[test]
    fn short_string2() {
        let hex = from_string("abcdefghijklmnopqrstuvwxyz".to_string());
        assert!(hex == "c3fcd3d76192e4007dfb496cca67e13b");
    }
    #[test]
    fn long_string1() {
        let hex = from_string(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        );
        assert!(hex == "d174ab98d277d9f5a5611c2c9f419d9f");
    }
    #[test]
    fn long_string2() {
        let hex = from_string(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                .to_string(),
        );
        assert!(hex == "57edf4a22be3c955ac49da2e2107b67a");
    }
}
