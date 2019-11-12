use wasm_bindgen::prelude::*;

extern crate crypto;

//rust crypto aes
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };


#[wasm_bindgen]
pub fn rust_crypto_aes_encrypt_decrypt(message: &str, key: &str, iv: &str){
    let encrypted = RustCryptoAes::encrypt(message, key, iv);
    let decrypted = RustCryptoAes::decrypt(encrypted, key, iv);
}
#[wasm_bindgen]
pub fn rust_crypto_aes_key_iv_setup(key: &str, iv : &str){
    let key = b"very secret key-the most secret.";
    let iv = b"my noncemy nonce";
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);
}


struct RustCryptoAes{

}

impl RustCryptoAes{
    pub fn encrypt(messag: &str, key: &str, iv: &str) -> Vec<u8>{
        let key = key.as_bytes();
        let (key, _right) = key.split_at(32);
        let iv = iv.as_bytes();
        let (iv, _right) = iv.split_at(16);
        let msg = messag.as_bytes();
        let ret = RustCryptoAes::private_encrypt(msg,key,iv).ok().unwrap();
        return ret;
    }
    pub fn decrypt(encrypted_data: Vec<u8>, key: &str, iv: &str) -> Vec<u8>{
        let key = key.as_bytes();
        let (key, _right) = key.split_at(32);
        let iv = iv.as_bytes();
        let (iv, _right) = iv.split_at(16);
        let data = encrypted_data.as_slice();
        let ret = RustCryptoAes::private_decrypt(data,key,iv).ok().unwrap();
        return ret;
    }
    //copy-pasted sample code from here, it's pretty long tho
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