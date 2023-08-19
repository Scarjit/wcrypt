#![allow(unused_variables)]
#![deny(clippy::unwrap_used)]

pub mod symmetric;
pub mod utils;

fn main() {
    use wasm_bindgen::prelude::*;

    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    #[wasm_bindgen]
    extern {
        fn alert(s: &str);
    }

    #[wasm_bindgen]
    pub fn greet() {
        use crate::utils::set_panic_hook;
        set_panic_hook();
        log!("Hello this is {} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    #[wasm_bindgen]
    pub fn encrypt(plaintext: &str, key: &str) -> Result<String,String> {
        use base64::Engine;
        let ciphertext = symmetric::encrypt(plaintext.as_bytes(), key).map_err(|e| e.to_string())?;
        Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(ciphertext))
    }

    #[wasm_bindgen]
    pub fn decrypt(ciphertext: &str, key: &str) -> Result<String,String> {
        use base64::Engine;
        let ciphertext = base64::engine::general_purpose::STANDARD_NO_PAD.decode(ciphertext.as_bytes()).expect("Invalid base64 ciphertext");
        let plaintext = symmetric::decrypt(&ciphertext, key).map_err(|e| e.to_string())?;
        String::from_utf8(plaintext).map_err(|e| e.to_string())
    }
}
