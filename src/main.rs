mod funcs; //Подключение файла funcs.rs
use funcs::consl::clean;
use funcs::input::{int_input, usize_input};
use funcs::password; //Подключение модуля password

fn main() {
    let action: u8 = password::menu();

    while action != 0 {
        //Очистка консоли
        clean();

        //Переменные для записи числа типа usize
        let length: usize;
        let mut str_length = String::new();

        //Длина пароля
        length = usize_input(&mut str_length);

        let password = password::generate(length);
        println!("{}", password);
    }
}
