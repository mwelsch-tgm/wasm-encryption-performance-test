extern crate c2_chacha;
//c2
use c2_chacha::stream_cipher::{NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek};
use c2_chacha::{ChaCha20};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn c2chacha20_encrypt_decrypt(message: &str, key: &str){
    let mut encrypt = C2ChaCha20Encryption::new(key);
    let v1 = encrypt.set_message(message);
    encrypt.encryption();
    let v2 =encrypt.decrypt();
}

#[wasm_bindgen]
pub fn c2chacha20_key_iv_setup(message: &str, key: &str){
    let mut encrypt = C2ChaCha20Encryption::new(key);
    //let v1 = encrypt.set_message(message);
}


pub struct C2ChaCha20Encryption{
    c2chacha20: ChaCha20,
    buffer: Vec<u8>,
}


impl C2ChaCha20Encryption{

    pub fn new(key: &str) -> C2ChaCha20Encryption{
        let key = b"very secret key-the most secret.";
        let iv = b"my nonce";
        let mut buffer = iv.to_vec();
        let c2chacha20 = ChaCha20::new_var(key, iv).unwrap();
        let encryption = C2ChaCha20Encryption {
            c2chacha20,
            buffer
        };
        return encryption;
    }

    pub fn set_message(&mut self, message: &str) -> Vec<u8>{
        let mut buffer = message.as_bytes();
        let mut buffer = buffer.to_vec();
        self.buffer = buffer;
        return self.buffer.clone();
    }
    pub fn encryption(&mut self) {
        self.c2chacha20.apply_keystream(&mut self.buffer);
    }

    pub fn decrypt(&mut self) -> Vec<u8>{
        self.c2chacha20.seek(0); // let's see if we need dis
        self.c2chacha20.apply_keystream(&mut self.buffer);
        return self.buffer.clone();

    }
}