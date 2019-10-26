mod utils;

use wasm_bindgen::prelude::*;

extern crate rand;
extern crate base64;
extern crate hex;
extern crate c2_chacha;
extern crate aes_gcm_siv;
extern crate blowfish;
extern crate block_modes;
extern crate crypto;
//for blowfishcbc
#[macro_use] extern crate hex_literal;

//c2
use c2_chacha::stream_cipher::{NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek};
use c2_chacha::{ChaCha20};


//rust crypto aes
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

//rust crypto blowfish
use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor};

//plain blowfish
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
//use blowfish::Blowfish; explicitly defined type for the block because Blowfish name is also used in rust-crypto

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
pub fn c2chacha20_encrypt_decrypt(message: &str, key: &str){
    let mut encrypt = C2ChaCha20Encryption::new(key);
    let v1 = encrypt.set_message(message);
    encrypt.encryption();
    let v2 =encrypt.decrypt();
}
#[wasm_bindgen]
pub fn blowfish_cbc_encrypt_decrypt(message: &str, key: &str){
    let cipher = BlowfishCbc::new_cipher(key);
    let mut buffer = BlowfishCbc::new_buffer(message);
    let mut vec = BlowfishCbc::encrypt(cipher,buffer,message.len());
    let cipher = BlowfishCbc::new_cipher(key);
    let decrypted = BlowfishCbc::decrypt(cipher,vec);
}
#[wasm_bindgen]
pub fn rust_crypto_aes_encrypt_decrypt(message: &str, key: &str){
    let encrypted = RustCryptoAes::encrypt(message, key);
    let decrypted = RustCryptoAes::decrypt(encrypted, key);
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
pub fn rust_crypto_blowfish_encrypt_decrypt(message: &str, key: &str){
    let  key = "very secret key-the most secret.";
    let message = "123456781234567812345678";
    let mut bl = RustCryptoBlowfish::new(key,message);
    bl.encrypt();
    let resultVec = bl.decrypt();
}

type CbcBlowfish = Cbc<blowfish::Blowfish, Pkcs7>;

pub struct BlowfishCbc{
}

impl BlowfishCbc{
    pub fn new_cipher(key: &str) -> CbcBlowfish{
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let iv = hex!("12345678912e3456");
        let cipher = CbcBlowfish::new_var(&key, &iv).unwrap();
        //let ciphertext = Vec::new();
        return cipher;
    }

    pub fn new_buffer(message: &str) -> [u8; 32]{
        let pos = message.len();
        let plaintext = message.as_bytes();
        let mut buffer = [0u8; 32];
        buffer[..pos].copy_from_slice(plaintext);
        return buffer;
    }


    pub fn encrypt(cipher: CbcBlowfish,mut buffer: [u8;32], plaintextLen: usize) -> Vec<u8>{
        let ciphertext = cipher.encrypt(&mut buffer, plaintextLen).unwrap();
        return ciphertext.to_vec();
    }

    pub fn decrypt(cipher: CbcBlowfish, mut  vec_buffer: Vec<u8>) -> Vec<u8>{
        let decrypted_ciphertext = cipher.decrypt(&mut vec_buffer).unwrap();
        return decrypted_ciphertext.to_vec();
    }

}


#[wasm_bindgen]
pub struct C2ChaCha20Encryption{
    c2chacha20: ChaCha20,
    buffer: Vec<u8>,
}

#[wasm_bindgen]
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

struct RustCryptoAes{

}
//copy-pasted sample code, it's pretty long tho
impl RustCryptoAes{
    pub fn encrypt(messag: &str, key: &str) -> Vec<u8>{
        let key = b"very secret key-the most secret.";
        let iv = b"my noncemy nonce";
        let msg = messag.as_bytes();
        let ret = RustCryptoAes::private_encrypt(msg,key,iv).ok().unwrap();
        return ret;
    }
    pub fn decrypt(encrypted_data: Vec<u8>, key: &str) -> Vec<u8>{
        let key = b"very secret key-the most secret.";
        let iv = b"my noncemy nonce";
        let data = encrypted_data.as_slice();
        let ret = RustCryptoAes::private_decrypt(data,key,iv).ok().unwrap();
        return ret;
    }
    fn private_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        Ok(final_result)
    }
    fn private_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        Ok(final_result)
    }
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
