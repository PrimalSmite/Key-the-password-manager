pub mod aes256 {
    use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

    pub fn encrypt(string: String) -> String {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let encrypted_string = mcrypt.encrypt_to_base64(&string);
        let encrypted_string = mcrypt.encrypt_to_base64::<U256>(&string);

        encrypted_string
    }

    pub fn decrypt(encrypted_string: &String) -> String {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let decrypt_string = mcrypt.decrypt_base64_to_string(encrypted_string).unwrap();

        decrypt_string
    }
}
