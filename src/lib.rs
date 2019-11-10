mod utils;

use wasm_bindgen::prelude::*;

extern crate aes_gcm_siv;

// blowfish
#[macro_use] extern crate hex_literal;

mod c2_chacha20;
mod blowfish_cbc;
mod crypto_aes;
mod crypto_blowfish;

// aesgcmsiv
use aes_gcm_siv::{Aes256GcmSiv, AesGcmSiv}; // Or `Aes128GcmSiv`
use aead::{Aead, NewAead, generic_array::GenericArray};

//general
use wasm_bindgen::__rt::core::ptr::null;

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
    alert("Hello, test-encrypting!");
}



#[wasm_bindgen]
pub fn aes_256_gcm_siv_encrypt_decrypt(message: &str, key: &str){
    let key = GenericArray::clone_from_slice(b"an example very very secret key.");
    let aead = Aes256GcmSiv::new(key);
    let nonce = GenericArray::from_slice(b"unique nonce"); // 96-bits; unique per message
    let ciphertext = aead.encrypt(nonce, message.as_bytes().as_ref()).expect("encryption failure!");
    let plaintext = aead.decrypt(nonce, ciphertext.as_ref()).expect("decryption failure!");
}

#[wasm_bindgen]
pub fn aes_256_gcm_siv_key_iv_setup(message: &str, key: &str){
    let key = GenericArray::clone_from_slice(b"an example very very secret key.");
    let aead = Aes256GcmSiv::new(key);
}

