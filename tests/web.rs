//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::File;
use js_sys::Uint8Array;
use md5_wasm::from_file;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn from_file_empty() {
    let file = File::new_with_u8_array_sequence(
        &Uint8Array::new_with_length(0).into(),
        "empty.txt",
    ).unwrap();
    let result = from_file(&file).await;
    assert_eq!(result, "d41d8cd98f00b204e9800998ecf8427e");
}

// #[wasm_bindgen_test]
// async fn from_file_a() {
//     let file = File::new_with_str_sequence(
//         &wasm_bindgen::JsValue::from_str("a"),
//         "a.txt", 
//     ).unwrap();
//     let result = from_file(&file).await;
//     assert_eq!(result, "0cc175b9c0f1b6a831c399e269772661");
// }

// #[wasm_bindgen_test]
// async fn from_file_a() {
//     let file = File::new_with_str_sequence(&JsValue::from_str("a"), "a.txt").unwrap();
//     let result = from_file(&file).await;
//     assert_eq!(result, "0cc175b9c0f1b6a831c399e269772661");
// }

// #[wasm_bindgen_test]
// async fn from_file_abc() {
//     let file = File::new_with_u8_array_sequence(
//         &JsValue::from(js_sys::Uint8Array::from("abc".as_bytes())),
//         "abc.txt",
//     ).unwrap();
//     let result = from_file(&file).await;
//     assert_eq!(result, "900150983cd24fb0d6963f7d28e17f72");
// }

// #[wasm_bindgen_test]
// async fn from_file_message_digest() {
//     let file = File::new_with_u8_array_sequence(
//         &Uint8Array::from("message digest".as_bytes()).into(),
//         "message_digest.txt",
//     ).unwrap();
//     let result = from_file(&file).await;
//     assert_eq!(result, "f96b697d7cb7938d525a2f31aaf161d0");
// }

// #[wasm_bindgen_test]
// async fn from_file_alphabet() {
//     let file = File::new_with_u8_array_sequence(
//         &Uint8Array::from("abcdefghijklmnopqrstuvwxyz".as_bytes()).into(),
//         "alphabet.txt",
//     ).unwrap();
//     let result = from_file(&file).await;
//     assert_eq!(result, "c3fcd3d76192e4007dfb496cca67e13b");
// }

// #[wasm_bindgen_test]
// async fn from_file_long_string1() {
//     let file = File::new_with_u8_array_sequence(
//         &Uint8Array::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes()).into(),
//         "long_string1.txt",
//     ).unwrap();
//     let result = from_file(&file).await;
//     assert_eq!(result, "d174ab98d277d9f5a5611c2c9f419d9f");
// }

// #[wasm_bindgen_test]
// async fn from_file_long_string2() {
//     let file = File::new_with_u8_array_sequence(
//         &Uint8Array::from("12345678901234567890123456789012345678901234567890123456789012345678901234567890".as_bytes()).into(),
//         "long_string2.txt",
//     ).unwrap();
//     let result = from_file(&file).await;
//     assert_eq!(result, "57edf4a22be3c955ac49da2e2107b67a");
// }