mod funcs; //Подключение файла funcs.rs
mod iomod;

use funcs::consl::clean;
use funcs::password; //Подключение моделя password
use iomod::input::input_line;

use crate::iomod::input::{read_int, to_u8};

fn main() {
    let action: u8 = password::menu();

    while action != 0 {
        if action == 1 {
            //Очистка консоли
            clean();

            //Переменные для записи числа типа i8
            let length: i8;
            let mut str_length = String::new();

            //Длина пароля
            println!("Введите длину пароля: ");
            length = int_input_i8(&mut str_length);

            let password = password::generate(length);
            println!("{}", password);
        }
    }
}
