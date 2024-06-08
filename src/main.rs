mod database;
mod finder;
mod funcs;
mod iomod;

use crate::iomod::input::read_i8;
//use database::db::{print_all, save};
use finder::file::{print_all_files, read_file, save_file};
use funcs::consl::pause;
use funcs::password;
use iomod::input; //Подключение моделя password

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

            match save_file(name, login, password) {
                Ok(saved) => println!("File saved: {:?}", saved),
                Err(err) => println!("Error: {}", err),
            };

            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 3 {
            println!("Выберите действие:\n(1) Показать всё\n(2) Показать определённоё");
            action = input::read_u8();

            while action != 1 && action != 2 {
                if action == 1 {
                    //database::db::print_all();
                    let _ = print_all_files();
                } else if action == 2 {
                    println!("Введите название сервиса:");
                    let name = input::input_line();
                    database::db::print_certain(&name);
                } else {
                    println!("Ошибка!");
                }
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
