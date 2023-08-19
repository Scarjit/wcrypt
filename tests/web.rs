//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn symmetric() {
    let text = "Hello World!";
    let key = "ABC";

    let ciphertext = crate::encrypt(text, key).unwrap();
    let plaintext = crate::decrypt(&ciphertext, key).unwrap();
    assert_eq!(text, plaintext)
}
