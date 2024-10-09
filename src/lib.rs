mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{File, FileReaderSync};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(file: web_sys::File) {
    let reader = match FileReaderSync::new() {
        Ok(reader) => reader,
        Err(e) => {
            alert(&format!("Error: {:?}", e));
            return;
        }
    };
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
}
