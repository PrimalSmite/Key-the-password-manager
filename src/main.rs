mod funcs; //Подключение файла funcs.rs
mod iomod;

use crate::iomod::input::{read_i8, read_u8};
use funcs::consl::{clean, pause};
use funcs::password; //Подключение моделя password

fn main() {
    let mut action: u8 = password::menu();

    while action != 0 {
        if action == 1 {
            /*
            //Очистка консоли
            clean();
            */

            //Переменные для записи числа типа i8
            let length: i8;
            //let mut str_length = String::new();

            //Длина пароля
            println!("Введите длину пароля: ");
            length = read_i8();

            let password = password::generate(length);
            println!("{}", password);

            //Пауза
            pause();

            action = password::menu();
        } else if action == 2 {
            println!("buba");
        }
    }
}
