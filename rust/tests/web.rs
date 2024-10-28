//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use md5::Md5;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn empty_string() {
    let input = "".as_bytes();
    let mut status = Md5::new();
    status.update(input);
    status.finalize();
    let hex = status.digest();
    print!("hex: {}", hex);
    assert!(hex == "d41d8cd98f00b204e9800998ecf8427e");
}
#[wasm_bindgen_test]
fn a() {
    let input = "a".as_bytes();
    let mut status = Md5::new();
    status.update(input);
    status.finalize();
    let hex = status.digest();
    print!("hex: {}", hex);
    assert!(hex == "0cc175b9c0f1b6a831c399e269772661");
}
#[wasm_bindgen_test]
fn abc() {
    let input = "abc".as_bytes();
    let mut status = Md5::new();
    status.update(input);
    status.finalize();
    let hex = status.digest();
    assert!(hex == "900150983cd24fb0d6963f7d28e17f72");
}
#[wasm_bindgen_test]
fn short_string1() {
    let input = "message digest".as_bytes();
    let mut status = Md5::new();
    status.update(input);
    status.finalize();
    let hex = status.digest();
    assert!(hex == "f96b697d7cb7938d525a2f31aaf161d0");
}
#[wasm_bindgen_test]
fn short_string2() {
    let input = "abcdefghijklmnopqrstuvwxyz".as_bytes();
    let mut status = Md5::new();
    status.update(input);
    status.finalize();
    let hex = status.digest();
    assert!(hex == "c3fcd3d76192e4007dfb496cca67e13b");
}
#[wasm_bindgen_test]
fn long_string1() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes();
    let mut status = Md5::new();
    status.update(input);
    status.finalize();
    let hex = status.digest();
    assert!(hex == "d174ab98d277d9f5a5611c2c9f419d9f");
}
#[wasm_bindgen_test]
fn long_string2() {
    let input = "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
        .as_bytes();
    let mut status = Md5::new();
    status.update(input);
    status.finalize();
    let hex = status.digest();
    assert!(hex == "57edf4a22be3c955ac49da2e2107b67a");
}
