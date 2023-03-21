pub mod encryption {
    use aes_gcm::{aes::Aes256};
    use aes_gcm_siv::{
        aead::{Aead, KeyInit},
        Aes256GcmSiv, Nonce, AesGcmSiv
    };
    use base64::Engine;

    pub fn encrypt(key: &str, s: &str) -> Result<String, aes_gcm::Error> {
        let cipher = create_cipher(key);
        let nonce = Nonce::from_slice(b"plaintext me"); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(nonce, s.as_ref())?;
        let mut output = Vec::new();
        output.resize(ciphertext.len() * 4 / 3 + 4, 0);
        let bytes_written = base64::engine::general_purpose::STANDARD.encode_slice(ciphertext.clone(), &mut output).unwrap();
        output.truncate(bytes_written);
        let output_string = hex::encode(String::from_utf8(output).unwrap());
        return Ok(output_string)
    }

    pub fn decrypt(key: &str, s: &str) -> String {
        let cipher = create_cipher(key);
        let nonce = Nonce::from_slice(b"plaintext me");
        let base64_vec = hex::decode(s).unwrap();
        let mut encrypted_bytes = Vec::new();
        encrypted_bytes.resize(base64_vec.len() * 4 / 3 + 4, 0);
        let decoded_bytes = base64::engine::general_purpose::STANDARD.decode_slice(base64_vec, &mut encrypted_bytes).unwrap();
        encrypted_bytes.truncate(decoded_bytes);
        let ciphertext = cipher.decrypt(nonce, encrypted_bytes.as_ref()).unwrap();
        let text = String::from_utf8(ciphertext).unwrap();
        return text;
    }

    // Make this return a key
    fn create_cipher(s: &str) -> AesGcmSiv<Aes256> {
        let mut mutable_vec = s.as_bytes().to_vec();
        mutable_vec.resize(32, 0);
        let arr: [u8; 32]  = mutable_vec.try_into().unwrap();
        return Aes256GcmSiv::new(arr.as_slice().into())
    }
}