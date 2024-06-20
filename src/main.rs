mod funcs; //Подключение файла funcs.rs
mod iomod;
mod crypto;
mod finder;

use std::{clone, fs};

use finder::file::{file_exists, save_file};
use crypto::aes256::{ask_for_crypt, check_password, decrypt, encrypt, hash_str, status_check, write_hash};
use iomod::input::input_line;
use crate::iomod::input::{read_i8, read_u8};
use funcs::consl::pause;
use funcs::password; //Подключение моделя password

fn main() {
    // Создает директорию с паролями, еслий такой нет
    if fs::create_dir("Passwords").is_err(){

    }
    let check = status_check(); 
    let mut action= password::menu();

    while action != 0 {
        if action == 1 {
            //Длина пароля
            println!("Введите длину пароля: ");
            let length: i8 = read_i8();

            //Создание пароля
            let password = password::generate(length);
            println!("{}", password);

            //Пауза
            pause();

            //Вызов меню
            action = password::menu();
        } else if action == 2 {
            // Ввод данных
            println!("Введите имя сервиса, логин и пароль\nИмя сервиса:");
            let name = input_line();
            println!("Логин:");
            let login = input_line();
            println!("Пароль:");
            let password = input_line();

            while check == 1 {
                println!("check:{}",check);
                if check == 1 {
                    ask_for_crypt();
                    let check = status_check();
                    println!("check:{}",check);
                } // Отсуствие обоих файлов
                else if check == 2  {save_file(&name, &login, &password)}// Наличие файла none.txt // Пропуск
                else { // Наличие файла pass.txt 
                    println!("Введите пароль:");
                    let pass = input_line();

                    // Цикл пока не будет выхода 
                    while pass != "Выход"{
                        // Если введен неверный пароль
                        if check_password(&pass) == false {
                            println!("Неверный пароль!");
                            println!("Повторите ввод пароля:");
                            let pass = input_line();
                        } else {
                            // Если введен верынй пароль
                            let encrypted_pass = encrypt(password.clone());
                            save_file(&name, &login, &encrypted_pass);
                        } 
                    }
                }
            }

            //Пауза
            pause();
            // Меню
            action = password::menu();
        }
    }

}
