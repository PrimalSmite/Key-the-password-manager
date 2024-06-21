mod funcs; //Подключение файла funcs.rs
mod iomod;
mod crypto;
mod finder;

use std::{clone, fs};

use finder::file::{change_login, file_exists, print_all_files, read_file, save_file};
use crypto::aes256::{ask_for_crypt, check_password, decrypt, encrypt, hash_str, status_check, write_hash};
use iomod::input::{self, input_line};
use crate::iomod::input::{read_i8, read_u8};
use funcs::consl::pause;
use funcs::password::{generate, menu, input_password}; //Подключение моделя password

fn crypto_config_check() -> i8{
    if status_check() == 1 { // Остутствие файлов, вопрос о шифровании
        ask_for_crypt();
        status_check()
    } else if status_check() == 2 { // Наличие none.txt, Пропуск
        2
    } else { // Наличие pass.txt
        3
    }
}

fn main() {
    // Создает директорию с паролями, еслий такой нет
    if fs::create_dir("Passwords").is_err(){
        // Пропуск
    }

    // Вызов функции если нет нужных файлов
    let check = crypto_config_check();
            
    let mut action= menu();

    while action != 0 {
        if action == 1 {
            //Длина пароля
            println!("Введите длину пароля: ");
            let length: i8 = read_i8();

            //Создание пароля
            let password = generate(length);
            println!("{}", password);

            //Пауза
            pause();

            //Вызов меню
            action = menu();
        } else if action == 2 {
            // Ввод данных
            println!("Введите имя сервиса, логин и пароль\nИмя сервиса:");
            let name = input_line();
            println!("Логин:");
            let login = input_line();
            println!("Пароль:");
            let password = input_line();

            if check == 2 {
                // Пропуск ввод пароля
                save_file(&name, &login, &password);
            } else {
                if input_password() == true {
                    save_file(&name, &login, &password);
                }
            }


            //Пауза
            pause();
            // Меню
            action = menu();
        } else if action == 3 {
            println!("(1) Показать список паролей\n(2) Показать конкретный пароль\n(0) Вернутсья");
            println!("Ввод:");
            let mut third_act = read_u8();

            while third_act != 0 {
                if third_act == 1 {
                    print_all_files();
                    println!("(1) Показать список паролей\n(2) Показать конкретный пароль\n(0) Вернутсья");
                    println!("Ввод:");
                    third_act = read_u8();
                } else if third_act == 2 {
                    println!("Введите имя сервиса");
                    let name = input_line();

                    if check == 2 {
                        // Пропуск ввода пароля
                        read_file(&name);
                    } else {
                        let mut pass = String::new();
                        //println!("Введите пароль:");
                        //pass = input_line();

                        while pass != "0"{
                            
                            println!("Введите пароль:");
                            pass = input_line();
                        
                            if check_password(&pass) == false{
                                println!("Неверный пароль! Введите пароль повторно, или введите 'Выход'");
                            } else {
                                read_file(&name);
                                break;
                            }
                        }
                        break;
                    }
                }
            }

            // Пауза
            pause();
            // Меню
            action == menu();
        } else if action == 4 {
            println!("Введите имя сервиса:");
            let name = input_line();

            println!("Введите новый логин:");
            let login = input_line();

            if check == 2 { // Пропуск
            change_login(&name, &login);

            } else {
                // Ввод пароля
                if input_password() == true {
                change_login(&name, &login);
                } else {
                    // Пропуск
                }
            }

            // Пауза
            pause();
            // Меню
            action = menu();
        } else if action == 5 {

        }
    }  

}
