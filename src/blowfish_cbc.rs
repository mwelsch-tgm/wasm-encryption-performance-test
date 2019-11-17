use wasm_bindgen::prelude::*;
//for blowfishcbc

extern crate blowfish;
extern crate block_modes;
//plain blowfish
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use blowfish::Blowfish;


#[wasm_bindgen]
pub fn blowfish_cbc_encrypt_decrypt(message: &str, key: &str, iv: &str){
    let cipher = BlowfishCbc::new_cipher(key, iv);
    let mut buffer = BlowfishCbc::new_buffer(message);
    let mut vec = BlowfishCbc::encrypt(cipher,buffer,message.len());
    let cipher = BlowfishCbc::new_cipher(key, iv);
    let decrypted = BlowfishCbc::decrypt(cipher,vec);
}

#[wasm_bindgen]
pub fn blowfish_cbc_key_iv_setup(key: &str, iv: &str){
    let cipher = BlowfishCbc::new_cipher(key, iv);
    //let mut buffer = BlowfishCbc::new_buffer(message);
}

#[wasm_bindgen]
pub fn blowfish_cbc_output_size(message: &str, key: &str, iv: &str) -> String{
    let cipher = BlowfishCbc::new_cipher(key, iv);
    let mut buffer = BlowfishCbc::new_buffer(message);
    let mut vec = BlowfishCbc::encrypt(cipher,buffer,message.len());
    let len1 = message.as_bytes().to_vec().len() as f32;
    let len2 = vec.len() as f32;
    let percent = len2/len1*100.0;
    return percent.to_string();
}

type CbcBlowfish = Cbc<Blowfish, Pkcs7>;

pub struct BlowfishCbc{

}

impl BlowfishCbc{
    pub fn new_cipher(key: &str, iv: &str) -> CbcBlowfish{
        /*let key = key.as_bytes();
        let (key, _right) = key.split_at(32);
        let iv = iv.as_bytes();
        let (iv, _right) = iv.split_at(16);*/
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let iv = hex!("12345678912e3456");
        let cipher = CbcBlowfish::new_var(&key, &iv).unwrap();
        //let ciphertext = Vec::new();
        return cipher;
    }

    pub fn new_buffer(message: &str) -> [u8; 128]{
        let pos = message.len();
        let plaintext = message.as_bytes();
        let mut buffer = [0u8; 128];
        buffer[..pos].copy_from_slice(plaintext);
        return buffer;
    }


    pub fn encrypt(cipher: CbcBlowfish,mut buffer: [u8;128], plaintextLen: usize) -> Vec<u8>{
        let ciphertext = cipher.encrypt(&mut buffer, plaintextLen).unwrap();
        return ciphertext.to_vec();
    }

    pub fn decrypt(cipher: CbcBlowfish, mut  vec_buffer: Vec<u8>) -> Vec<u8>{
        let decrypted_ciphertext = cipher.decrypt(&mut vec_buffer).unwrap();
        return decrypted_ciphertext.to_vec();
    }

}
