mod funcs; //Подключение файла funcs.rs
mod iomod;

use std::io::{self, stdin};

use crate::iomod::input::{read_i8, read_u8};
use funcs::consl::pause;
use funcs::password; //Подключение моделя password

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
            println!("buba");
        }
    }
}
