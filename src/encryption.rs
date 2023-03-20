pub mod encryption {
    use aes_gcm_siv::{
        aead::{Aead, KeyInit},
        Aes256GcmSiv, Nonce
    };
    use base64::Engine;

    pub fn encrypt(key: &str, s: &str) -> Result<String, aes_gcm::Error> {
        let cipher = Aes256GcmSiv::new(key_maker(key).into());
        let nonce = Nonce::from_slice(b"plaintext me"); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(nonce, s.as_ref())?;
        let mut output = Vec::new();
        output.resize(ciphertext.len() * 4 / 3 + 4, 0);
        let bytes_written = base64::engine::general_purpose::STANDARD.encode_slice(ciphertext.clone(), &mut output).unwrap();
        output.truncate(bytes_written);
        let output_string = hex::encode(String::from_utf8(output).unwrap());
        return Ok(output_string)
    }

    pub fn decrypt(key: &str, s: &str) {
        let cipher = Aes256GcmSiv::new(key_maker(key).try_into()?);
    }

    // Make this return a key
    fn key_maker(s: &str) -> [u8; 256] {
        let mut mutable_vec = s.as_bytes().to_vec();
        &mutable_vec.resize(32, 0);
        let arr: [u8; 256] = mutable_vec.try_into().unwrap();
        return arr;
    }
}