mod finder;
mod funcs;
mod iomod;

use iomod::input::{input_line, read_i8};
use finder::file::{change_login, change_password, print_all_files, read_file, rm_file, save_file};
use funcs::consl::pause;
use funcs::password;

fn main() {
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
            // Ввод данных пользователя
            println!("Введите название сервиса: ");
            let name = input_line();
            println!("Введите логин: ");
            let login = input_line();
            println!("Введите пароль: ");
            let password = input_line();

            match save_file(name, login, password) {
                Ok(saved) => println!("File saved: {:?}", saved),
                Err(err) => println!("Error: {}", err),
            };

            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 3 {
            println!("Выберите действие:\n(1) Показать всё\n(2) Показать определённое\n(0) Меню");
            action = iomod::input::read_u8();

            if action == 1{
                println!("(1) Показать всё");
                let _ = print_all_files();
            } else if action == 2{
                println!("(2) Показать определённое");
                println!("Введите имя сервиса");
                let name = input_line();
                read_file(name);
            } else if action == 0{
                
            } else {
                println!("Ошибка! Выберите корректное действие!");
            }

            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 4 {
            println!("Введите название сервиса:");
            let name = input_line();

            println!("Введите новый логин");
            let new = input_line();

            change_login(name, new);
            //Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 5 {
            // Изменение пароля
            println!("Введите название сервиса:");
            let name = input_line();

            println!("Введите новый пароль:");
            let new = input_line();

            change_password(name, new);
            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        } else if action == 6 {
            // Удаление пароля
            println!("Введите название сервиса:");
            let name = input_line();
            rm_file(name);
            // Пауза
            pause();
            // Вызов меню
            action = password::menu();
        }
    }
}
