extern crate chacha20poly1305;
use chacha20poly1305::ChaCha20Poly1305;
use aead::{Aead, NewAead, generic_array::GenericArray};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn chacha20_poly1305_encrypt_decrypt(message: &str, key: &str, iv: &str){
    let key = key.as_bytes();
    let (key, _right) = key.split_at(32);
    let key = GenericArray::clone_from_slice(key); // 32-bytes
    let aead = ChaCha20Poly1305::new(key);
    let iv = iv.as_bytes();
    let (iv, _right) = iv.split_at(12);
    let nonce = GenericArray::from_slice(iv); // 12-bytes; unique per message
    let ciphertext = aead.encrypt(nonce, b"plaintext message".as_ref()).expect("encryption failure!");
    let plaintext = aead.decrypt(nonce, ciphertext.as_ref()).expect("decryption failure!");
}
#[wasm_bindgen]
pub fn chacha20_poly1305_key_iv_setup(key: &str, iv: &str){
    let key = key.as_bytes();
    let (key, _right) = key.split_at(32);
    let key = GenericArray::clone_from_slice(key); // 32-bytes
    let aead = ChaCha20Poly1305::new(key);
    let iv = iv.as_bytes();
    let (iv, _right) = iv.split_at(12);
    //let nonce : GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>> = GenericArray::from_slice(iv); // 12-bytes; unique per message
}

#[wasm_bindgen]
pub fn chacha20_poly1305_output_size(message: &str, key: &str, iv: &str) -> String{
    let key = key.as_bytes();
    let (key, _right) = key.split_at(32);
    let key = GenericArray::clone_from_slice(key); // 32-bytes
    let aead = ChaCha20Poly1305::new(key);
    let iv = iv.as_bytes();
    let (iv, _right) = iv.split_at(12);
    let nonce = GenericArray::from_slice(iv); // 12-bytes; unique per message
    let ciphertext = aead.encrypt(nonce, message.as_bytes().as_ref()).expect("encryption failure!");
    let len1 = message.as_bytes().to_vec().len() as f32;
    let len2 = ciphertext.len() as f32;
    let percent = len2/len1*100.0;
    return percent.to_string();
}