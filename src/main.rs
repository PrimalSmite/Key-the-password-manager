mod database;
mod funcs; //Подключение файла funcs.rs
mod iomod;

use crate::iomod::input::read_i8;
use database::db::{print_all, save};
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

            // Сохранение данных в базу данных
            match database::db::save(name, login, password) {
                Err(err) => println!("Insert failed: {}", err),
                Ok(inserted) => println!("Inserted data: {:?}", inserted),
            }

            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 3 {
            let st = database::db::print_all();

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
