extern crate crypto;

//rust crypto blowfish
use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor};


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust_crypto_blowfish_encrypt_decrypt(message: &str, key: &str){
    let  key = "very secret key-the most secret.";
    let message = "123456781234567812345678";
    let mut bl = RustCryptoBlowfish::new(key,message);
    bl.encrypt();
    let resultVec = bl.decrypt();
}




struct RustCryptoBlowfish{
    message: String,
    blfish: Blowfish,
    encrypted: Vec<u8>,
}
impl RustCryptoBlowfish{

    pub fn new(key: &str, message: &str) -> RustCryptoBlowfish{
        let key = key.as_bytes();
        //let key = b"very secret key-the most secret.";
        let blfish = Blowfish::new(key);
        let encrypted  =  Vec::from(message);
        let message = String::from(message);
        let obj = RustCryptoBlowfish{
            message,
            blfish,
            encrypted
        };
        return obj;
    }

    pub fn set_message(&mut self, message: &str){
        //make sure message is a multiple of 8 long
        let mut test = String::from(message);
        while test.len()%8 !=0 {
            test.push_str(" ");
        }
        self.message = test;
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
