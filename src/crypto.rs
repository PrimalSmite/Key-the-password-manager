pub mod aes256 {
    use std::collections::hash_map::DefaultHasher;
    use std::fs::File;
    use std::hash::{Hash, Hasher};
    use std::io::{Bytes, Read, Write};
    use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

    use crate::finder::file::DIR_PATH;

    // Шифрование
    pub fn encrypt(string: String) -> String {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let encrypted_string = mcrypt.encrypt_to_base64(&string);

        encrypted_string
    }

    // Дешифрование 
    pub fn decrypt(encrypted_string: &String) -> String {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let decrypt_string = mcrypt.decrypt_base64_to_string(encrypted_string).unwrap();

        decrypt_string
    }

    // Хеширование 
    pub fn hash_str(data: &String) -> String{
        // Хеширование строки
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish().to_string()
    }

    // Запись хеша
    pub fn write_hash(hash: &String){
        let path = format!("{}/pass.txt", DIR_PATH);
        let mut f = File::create(path).unwrap();

        f.write_all(hash.as_bytes()).unwrap();
    }

    // Проверка
    pub fn check_hash(a: String){
        let path = format!("{}/pass.txt", DIR_PATH);
        let mut file = File::open(path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let check = hash_str(&a); 

        if contents == check.to_string(){
            println!("ura");
        } else {
            println!("da blyat");
            println!("{}",contents);
        }

    }
}
