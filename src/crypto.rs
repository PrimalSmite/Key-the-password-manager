pub mod aes256 {
    use std::collections::hash_map::DefaultHasher;
    use std::fs::{File};
    use std::hash::{Hash, Hasher};
    use std::io::{Bytes, Read, Write};
    use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

    use crate::finder::file::{DIR_PATH, file_exists};
    use crate::iomod::input::{input_line, read_u8};

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
        let mut f = File::create("Passwords/pass.txt").unwrap();

        f.write_all(hash.as_bytes()).unwrap();
    }

    // Проверка
    pub fn check_password(a: &String) -> bool{
        // Открытие файла
        let path = format!("{}/pass.txt", DIR_PATH);
        let mut file = File::open(path).unwrap();

        // Чтение файла
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Хеширование введенного пароля
        let check = hash_str(&a); 

        // Сравнение    
        if contents == check.to_string(){
            true
        } else {
            false
        }

    }

    // Вопрос
    pub fn ask_for_crypt(){
        println!("Хотите использовать шифрование для хранения паролей?\n(1) Да\n(2) Нет");
        let ask_crypt = read_u8();
        
        if ask_crypt == 1{
            println!("Создайте паролй, который будет использоватся для подтверждения действий:");
            let password = input_line(); // Ввод пароля
            let hash_pass = hash_str(&password); // Хеширование пароля
            write_hash(&hash_pass); // Запись хеша в файл
            println!("Ваш пароль хранится в файле pass.txt в директории Passwords в зашифровонном виде.");
        } else if ask_crypt == 2 {
            println!("Вы можете изменить решение, удалив файл none.txt в дикректории Passwords");
            File::create_new("Passwords/none.txt").unwrap(); // Создание файла, для пометки что шифрования нет
        }
    }

    // Проверка наличия файла pass.txt и none.txt
    pub fn status_check() -> i8{
        if file_exists() == 0{ // Отсутствие обоих файлов
            1
        } else if file_exists() == -1 { // Наличие файла none.txt
            // Пропуск
            2
        } else { // Наличие файла pass.txt
            3
        }

    }

}
