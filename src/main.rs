mod crypto;
mod funcs; //Подключение файла funcs.rs
mod iomod;

use std::io::{self, stdin};

use crate::iomod::input::{read_i8, read_u8};
use crypto::aes256::{self, encrypt};
use funcs::consl::pause;
use funcs::password; //Подключение моделя password
mod database;

use database::db::{self, print_all, save};
use iomod::input::{self, input_line}; //Подключение моделя password

fn main() {
    let mut action: u8 = password::menu();

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
            // Ввод данных пользователя
            println!("Введите название сервиса: ");
            let name = iomod::input::input_line();
            println!("Введите логин: ");
            let login = input::input_line();
            println!("Введите пароль: ");
            let password = input::input_line();
            let encrypted_pass = encrypt(password);

            // Сохранение данных в базу данных
            database::db::save(name, login, encrypted_pass);
            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 3 {
            println!("Выберите действие:\n(1) Показать всё\n(2) Показать определённое");
            action = input::read_u8();
            if action == 1 {
                database::db::print_all();
            } else if action == 2 {
                println!("Введите название сервиса:");
                let name = input_line();
                db::print_certain(&name);
            }

            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 4 {
            //Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 5 {
            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 6 {
            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        }
    }
}
