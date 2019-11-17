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
pub fn aes_256_gcm_siv_encrypt_decrypt(message: &str, key: &str, iv: &str){
    let key = key.as_bytes();
    let (key, _right) = key.split_at(32);
    let key = GenericArray::clone_from_slice(key);
    let aead = Aes256GcmSiv::new(key);
    let iv = iv.as_bytes();
    let (iv, _right) = iv.split_at(12);
    let nonce = GenericArray::from_slice(iv); // 96-bits; unique per message
    let ciphertext = aead.encrypt(nonce, message.as_bytes().as_ref()).expect("encryption failure!");
    let plaintext = aead.decrypt(nonce, ciphertext.as_ref()).expect("decryption failure!");
}

#[wasm_bindgen]
pub fn aes_256_gcm_siv_key_iv_setup(key: &str, iv: &str){
    let key = key.as_bytes();
    let (key, _right) = key.split_at(32);
    let key = GenericArray::clone_from_slice(key);
    let aead = Aes256GcmSiv::new(key);
    let iv = iv.as_bytes();
    let (iv, _right) = iv.split_at(12);
   // let nonce : GenericArray<u8, aead::generic_array::ArrayLength<u8>> = GenericArray::from_slice(iv); // 96-bits; unique per message
}

#[wasm_bindgen]
pub fn aes_256_gcm_siv_output_size(message: &str, key: &str, iv: &str) -> String{
    let key = key.as_bytes();
    let (key, _right) = key.split_at(32);
    let key = GenericArray::clone_from_slice(key);
    let aead = Aes256GcmSiv::new(key);
    let iv = iv.as_bytes();
    let (iv, _right) = iv.split_at(12);
    let nonce = GenericArray::from_slice(iv); // 96-bits; unique per message
    let ciphertext = aead.encrypt(nonce, message.as_bytes().as_ref()).expect("encryption failure!");
    let len1 = message.as_bytes().to_vec().len() as f32;
    let len2 = ciphertext.len() as f32;
    let percent = len2/len1*100.0;
    return percent.to_string();
}