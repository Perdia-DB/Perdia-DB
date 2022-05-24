use aes::{cipher::{generic_array::GenericArray, typenum::{UInt, UTerm}, consts::{B1, B0}, KeyInit, BlockEncrypt, BlockDecrypt}, Aes128};
use crypto::{sha3::Sha3, digest::Digest};

/// An abstraction for the underlying AES128 and Shake128 ciphers
/// 
/// Key operates as follows:
/// 
/// The [`Key`] generates a Shake128 hash of the passed raw key([`Vec<u8>`]),
/// it then takes encrypts or decrypts a certain number of bytes ([`Vec<u8>`]) using AES128 and returns the result.
pub struct Key {
    key: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>>,
    cipher: Aes128,
}

impl Key {
    /// Hashing the raw key using Shake128
    pub fn new(key: Vec<u8>) -> Self {
        let mut hasher = Sha3::shake_128();
        hasher.input(&key);
        let mut key_array = [0u8; 16];
        hasher.result(&mut key_array);
        let key = GenericArray::from(key_array);
        let cipher = Aes128::new(&key.into());
        Self { key, cipher }
    }

    // TODO: Improve performance
    /// Encrypt some bytes([`Vec<u8>`]) using AES128.
    pub fn encrypt(&self, mut bytes: Vec<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::with_capacity(bytes.len());
        'encrypt_loop: 
        loop {
            if bytes.len() <= 16 {
                for _ in bytes.len()..16 {
                    bytes.push(0);
                }
                let mut block: [u8; 16] = [0u8; 16];
                block.copy_from_slice(bytes.as_slice());
                let mut block = GenericArray::from(block);
                self.cipher.encrypt_block(&mut block);
                output.extend(&block);
                break 'encrypt_loop;
            } else {
                let (slice, b) = bytes.split_at(16);
                let mut block: [u8; 16] = [0u8; 16];
                block.copy_from_slice(slice);
                let mut block = GenericArray::from(block);
                self.cipher.encrypt_block(&mut block);
                output.extend(&block);
                bytes = b.to_vec();
            }

        }
        output
    }

    // TODO: Improve performance
    /// Decrypt the given bytes([`Vec<u8>`]) using AES128.
    pub fn decrypt(&self, mut bytes: Vec<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::with_capacity(bytes.len());
        'encrypt_loop: 
        loop {
            if bytes.len() == 0 {
                break 'encrypt_loop;
            } else {
                let (slice, b) = bytes.split_at(16);
                let mut block: [u8; 16] = [0u8; 16];
                block.copy_from_slice(slice);
                let mut block = GenericArray::from(block);
                self.cipher.decrypt_block(&mut block);
                output.extend(&block);
                bytes = b.to_vec();
            }

        }
        output
    }

}

impl From<Vec<u8>> for Key {
    fn from(key: Vec<u8>) -> Self {
        Self::new(key)
    }
}

impl From<String> for Key {
    fn from(key: String) -> Self {
        let key = key.into_bytes();
        Self::new(key)
    }
}

/// Make new [`Key`] from inner key.
impl Into<GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>>> for Key {
    fn into(self) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>> {
        self.key
    }
}