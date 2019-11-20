extern crate crypto;

//rust crypto blowfish
use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor};


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust_crypto_blowfish_encrypt_decrypt(message: &str, key: &str){
    let mut bl = RustCryptoBlowfish::new(key,message);
    bl.encrypt();
    let resultVec = bl.decrypt();
    assert_eq!(message.as_bytes().to_vec(),resultVec);
}
#[wasm_bindgen]
pub fn rust_crypto_blowfish_key_iv_setup(key: &str){
    let mut bl = RustCryptoBlowfish::new(key,"");
}

#[wasm_bindgen]
pub fn rust_crypto_blowfish_output_size(message: &str, key: &str) -> String{
    let mut bl = RustCryptoBlowfish::new(key,message);
    bl.encrypt();
    let len1 = message.as_bytes().to_vec().len() as f32;
    let len2 = bl.get_encrypted().len() as f32;
    let percent = len2/len1*100.0;
    return percent.to_string();
}

struct RustCryptoBlowfish{
    message: String,
    blfish: Blowfish,
    encrypted: Vec<u8>,
}
impl RustCryptoBlowfish{

    pub fn new(key: &str, message: &str) -> RustCryptoBlowfish{
        let key = key.as_bytes();
        let (key, _right) = key.split_at(32);
        let blfish = Blowfish::new(key);
        let encrypted  =  Vec::from(message);
        let message = String::from(message);
        let message2 = message.clone();
        let mut obj = RustCryptoBlowfish{
            message,
            blfish,
            encrypted
        };
        obj.set_message(message2);
        return obj;
    }

    pub fn set_message(&mut self, message: String){
        //make sure message is a multiple of 8 long
        let mut test = String::from(message);
        while test.len()%8 !=0 {
            test.push_str(" ");
        }
        self.message = test;
    }

    pub fn get_message(&mut self) -> String{
        return self.message.clone();
    }

    pub fn get_encrypted(&mut self) -> Vec<u8>{
        return self.encrypted.clone();
    }

    pub fn encrypt(&mut self){
        self.encrypted = Vec::new();
        let  output =  &mut [1,2,3,4,5,6,7,8];
        for i in 1..(self.message.as_bytes().len()/8 +1)  {
            let to_encrypt = self.message.as_bytes().get((i-1)*8..(i*8)).unwrap();
            self.blfish.encrypt_block(to_encrypt, output);
            self.encrypted.append(output.to_vec().as_mut());
        }
    }

    pub fn decrypt(&mut self) -> Vec<u8>{
        let mut decrypted = Vec::new();
        let  output =  &mut [1,2,3,4,5,6,7,8];
        for i in 1..(self.encrypted.len()/8 +1){
            let to_decrypt =self.encrypted.as_slice().get((i-1)*8..(i*8)).unwrap();
            self.blfish.decrypt_block(to_decrypt, output);
            decrypted.append(output.to_vec().as_mut());
        }
        return decrypted;
    }
}
