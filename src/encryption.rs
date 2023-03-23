pub mod encryption {
    use aes_gcm::{aes::Aes256};
    use aes_gcm_siv::{
        aead::{Aead, KeyInit},
        Aes256GcmSiv, Nonce, AesGcmSiv
    };
    use base64::Engine;

    pub fn encrypt(key: &str, s: &str, nonce: &mut Vec<u8>, encode: bool, binary: bool) -> Result<String, aes_gcm::Error> {
        let cipher = create_cipher(key);

        nonce.resize(12, 0);
        let nonce = Nonce::from_slice(&nonce);

        let ciphertext = cipher.encrypt(nonce, s.as_ref())?;

        let mut output = Vec::new();
        output.resize(ciphertext.len() * 4 / 3 + 4, 0);
        let bytes_written = base64::engine::general_purpose::STANDARD.encode_slice(ciphertext.clone(), &mut output).unwrap();
        output.truncate(bytes_written);

        let output_string = hex::encode(String::from_utf8(output).unwrap());

        if encode {
            return Ok(convert_to_clown(&output_string, binary))
        }

        return Ok(output_string)
    }

    pub fn decrypt(key: &str, s: &str, nonce: &mut Vec<u8>, encode: bool, binary: bool) -> String {

        let mut mutable_string = s.to_string();

        if encode {
            mutable_string = convert_from_clown(&mutable_string, binary);
        }

        let cipher = create_cipher(key);

        nonce.resize(12, 0);
        let nonce = Nonce::from_slice(&nonce);

        let base64_vec = hex::decode(mutable_string).unwrap();
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

    fn get_clown_convertion() -> [(&'static str, &'static str); 16] {
        return [("a", "🥸"), ("b", "🥛"), ("c", "🗿"), ("d", "🤨"), ("e", "😐"), ("f", "😏"), ("0", "🤡"), ("1", "🤓"), ("2", "🫁"), ("3", "🤯"), ("4", "📮"), ("5", "🐄"), ("6", "🥌"), ("7", "💩"), ("8", "🤠"), ("9", "🥴")];
    }

    fn convert_to_clown(source_string: &str, binary: bool) -> String {
        let mut target_string: String = hex::encode(source_string);
        if binary {
            let hex_string = target_string.clone();
            target_string = "".to_string();
            for character in hex_string.into_bytes() {
                target_string += &format!("{:0>8}", format!("{:b}", character))
            }

        }

        for conversion in get_clown_convertion() {
            target_string = target_string.replace(conversion.0, conversion.1)
        }
        

        return target_string.to_string()
    }

    fn convert_from_clown(source: &str, binary: bool) -> String {
        let mut declowned_string = source.to_string();
        let mut target_string;

        for conversion in get_clown_convertion() {
            declowned_string = declowned_string.replace(conversion.1, conversion.0);
        }

        if binary {
            target_string = String::from_utf8((0..declowned_string.len()).step_by(8).map(|i| u8::from_str_radix(&declowned_string[i..i + 8], 2).unwrap()).collect()).unwrap();
        } else {
            target_string = declowned_string
        }

        target_string = String::from_utf8(hex::decode(target_string).unwrap()).unwrap();

        return target_string
    }
}