//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn symmetric() {
    let text = "Hello World!".as_bytes();
    let key = "ABC";

    let ciphertext = wcrypt::symmetric::encrypt(text, key).unwrap();
    let plaintext = wcrypt::symmetric::decrypt(&ciphertext, key).unwrap();
    assert_eq!(text, plaintext)
}
