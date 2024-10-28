//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use md5::md5_from_u8_array;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn from_file_empty() {
    let result = md5_from_u8_array("".as_bytes());
    assert_eq!(result, "d41d8cd98f00b204e9800998ecf8427e");
}

#[wasm_bindgen_test]
fn from_file_a() {
    let result = md5_from_u8_array("a".as_bytes());
    assert_eq!(result, "0cc175b9c0f1b6a831c399e269772661");
}

#[wasm_bindgen_test]
fn from_file_abc() {
    let result = md5_from_u8_array("abc".as_bytes());
    assert_eq!(result, "900150983cd24fb0d6963f7d28e17f72");
}

#[wasm_bindgen_test]
fn from_file_message_digest() {
    let result = md5_from_u8_array("message digest".as_bytes());
    assert_eq!(result, "f96b697d7cb7938d525a2f31aaf161d0");
}

#[wasm_bindgen_test]
fn from_file_alphabet() {
    let result = md5_from_u8_array("abcdefghijklmnopqrstuvwxyz".as_bytes());
    assert_eq!(result, "c3fcd3d76192e4007dfb496cca67e13b");
}

#[wasm_bindgen_test]
fn from_file_long_string1() {
    let result = md5_from_u8_array(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes(),
    );
    assert_eq!(result, "d174ab98d277d9f5a5611c2c9f419d9f");
}

#[wasm_bindgen_test]
fn from_file_long_string2() {
    let result = md5_from_u8_array(
        "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
            .as_bytes(),
    );
    assert_eq!(result, "57edf4a22be3c955ac49da2e2107b67a");
}
