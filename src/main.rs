mod funcs; //Подключение файла funcs.rs
mod iomod;
mod crypto;
mod finder;

use std::io::{self, stdin};

use crypto::aes256::{check_hash, decrypt, encrypt, hash_str, write_hash};
use iomod::input::input_line;
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
            let a = input_line(); 
            let b = hash_str(&a);
            println!("{}",b);
            write_hash(&b);

            check_hash(a);

        }
    }
}
