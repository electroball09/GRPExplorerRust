use cipher::array::Array;
use twofish::Twofish;
use twofish::cipher::{BlockCipherEncrypt, KeyInit};


pub fn twofish_decrypt_cfb1(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let engine = Twofish::new_from_slice(key).expect("Invalid key length");
    
    let mut shift_register = u128::from_be_bytes(iv.try_into().unwrap());
    let mut plaintext = Vec::with_capacity(ciphertext.len());

    for &c_byte in ciphertext {
        let mut p_byte = 0u8;

        for bit in (0..8).rev() {
            let mut encrypted_sr = Array::from(shift_register.to_be_bytes());
            engine.encrypt_block(&mut encrypted_sr);

            let key_bit = (encrypted_sr[0] >> 7) & 1;
            let cipher_bit = (c_byte >> bit) & 1;
            let plain_bit = key_bit ^ cipher_bit;

            p_byte |= plain_bit << bit;

            shift_register = (shift_register << 1) | (cipher_bit as u128);
        }
        plaintext.push(p_byte);
    }

    plaintext
}


pub fn twofish_encrypt_cfb1(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let engine = Twofish::new_from_slice(key).expect("Invalid key length");
    
    let mut shift_register = u128::from_be_bytes(iv.try_into().unwrap());
    let mut ciphertext = Vec::with_capacity(plaintext.len());

    for &p_byte in plaintext {
        let mut c_byte = 0u8;

        for bit in (0..8).rev() {
            let mut encrypted_sr = Array::from(shift_register.to_be_bytes());
            engine.encrypt_block(&mut encrypted_sr);

            let key_bit = (encrypted_sr[0] >> 7) & 1;            
            let plain_bit = (p_byte >> bit) & 1;            
            let cipher_bit = key_bit ^ plain_bit;

            c_byte |= cipher_bit << bit;

            shift_register = (shift_register << 1) | (cipher_bit as u128);
        }
        ciphertext.push(c_byte);
    }

    ciphertext
}