mod funcs; //Подключение файла funcs.rs
mod iomod;
mod crypto;
mod finder;

use std::fs;

use finder::file::{change_login, change_password, file_exists, print_all_files, read_file, rm_file, save_file};
use crypto::aes256::{ask_for_crypt, check_password, decrypt, encrypt, hash_str, status_check, write_hash};
use iomod::input::input_line;
use crate::iomod::input::{read_i8, read_u8};
use funcs::consl::pause;
use funcs::password::{self, generate, input_password, menu}; //Подключение моделя password

pub fn crypto_config_check() -> i8{
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
                    save_file(&name, &login, &encrypt(&password));
                }
            }

            //Пауза
            pause();
            // Меню
            action = menu();
        } else if action == 3 {
            let mut act:u8 = 1; 

            while act != 0 {
                println!("\n(1) Показать список паролей\n(2) Показать конкретный пароль\n(0) Вернутсья");
                println!("Ввод:");
                act = read_u8();

                if act == 1 {
                    println!(""); // Пустая строка для пространства
                    print_all_files();
                } else if act == 2 {
                    println!("Введите имя сервиса:");
                    let name = input_line();
                    
                    if check == 2 {
                        // Пропуск ввода пароля
                        read_file(&name);
                    } else {
                        // Ввод пароля
                        if input_password() == true {
                            read_file(&name);
                            break;
                        }
                    }
                } else if act != 1 && act != 2 && act != 0 {
                    println!("\nВыберите корректное действие!");
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
                } 
            }

            // Пауза
            pause();
            // Меню
            action = menu();
        } else if action == 5 {
            println!("Введите имя сервиса:");
            let name = input_line();

            println!("Введите новый пароль:");
            let new = input_line();

            if check == 2 {
                // Пропуск ввода пароля
                change_password(&name, &new);
            } else {
                // Ввод пароля
                if input_password() == true {
                    change_password(&name, &new);
                }
            }

            // Пауза
            pause();
            // Меню
            action == password::menu();
        } else if action == 6 {
            println!("Введите имя сервиса:");
            let name = input_line();

            println!("Введите 'ПОДТВЕРДИТЬ' для продолжения:");
            let confirmation = input_line();

            if confirmation.trim() != "ПОДТВЕРДИТЬ" {
                println!("Возврать в главное меню...");
            } else {
                if check == 2 {
                    // Пропуск ввода пароля
                    rm_file(&name);
                } else {
                    // Ввод пароля
                    if input_password() == true {
                        rm_file(&name);
                    }
                }
            }

            // Пауза
            pause();
            // Меню
            action == password::menu();
        } 
    }  

}
