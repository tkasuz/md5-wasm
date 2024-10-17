//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::Uint8Array;
use md5::{md5_from_file, md5_from_string};
use wasm_bindgen_test::*;
use web_sys::File;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn from_file_empty() {
    let file =
        File::new_with_u8_array_sequence(&Uint8Array::new_with_length(0).into(), "empty.txt")
            .unwrap();
    let result = md5_from_file(&file).await;
    assert_eq!(result, "d41d8cd98f00b204e9800998ecf8427e");
}

#[wasm_bindgen_test]
async fn from_file_a() {
    let file = web_sys::File::new_with_str_sequence(
        &js_sys::Array::from(&js_sys::JsString::from("a")),
        "a.txt",
    )
    .unwrap();
    let result = md5_from_file(&file).await;
    assert_eq!(result, "0cc175b9c0f1b6a831c399e269772661");
}

#[wasm_bindgen_test]
async fn from_file_abc() {
    let file = web_sys::File::new_with_str_sequence(
        &js_sys::Array::from(&js_sys::JsString::from("abc")),
        "abc.txt",
    )
    .unwrap();
    let result = md5_from_file(&file).await;
    assert_eq!(result, "900150983cd24fb0d6963f7d28e17f72");
}

#[wasm_bindgen_test]
async fn from_file_message_digest() {
    let file = web_sys::File::new_with_str_sequence(
        &js_sys::Array::from(&js_sys::JsString::from("message digest")),
        "message_digest.txt",
    )
    .unwrap();
    let result = md5_from_file(&file).await;
    assert_eq!(result, "f96b697d7cb7938d525a2f31aaf161d0");
}

#[wasm_bindgen_test]
async fn from_file_alphabet() {
    let file = web_sys::File::new_with_str_sequence(
        &js_sys::Array::from(&js_sys::JsString::from("abcdefghijklmnopqrstuvwxyz")),
        "abcdefghijklmnopqrstuvwxyz.txt",
    )
    .unwrap();
    let result = md5_from_file(&file).await;
    assert_eq!(result, "c3fcd3d76192e4007dfb496cca67e13b");
}

#[wasm_bindgen_test]
async fn from_file_long_string1() {
    let file = web_sys::File::new_with_str_sequence(
        &js_sys::Array::from(&js_sys::JsString::from(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        )),
        "long1.txt",
    )
    .unwrap();
    let result = md5_from_file(&file).await;
    assert_eq!(result, "d174ab98d277d9f5a5611c2c9f419d9f");
}

#[wasm_bindgen_test]
async fn from_file_long_string2() {
    let file = web_sys::File::new_with_str_sequence(
        &js_sys::Array::from(&js_sys::JsString::from(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        )),
        "long2.txt",
    )
    .unwrap();
    let result = md5_from_file(&file).await;
    assert_eq!(result, "57edf4a22be3c955ac49da2e2107b67a");
}

#[wasm_bindgen_test]
fn from_string() {
    assert!(
        md5_from_string(js_sys::JsString::from("message digest"))
            == "f96b697d7cb7938d525a2f31aaf161d0"
    );
}
