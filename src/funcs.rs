//Модуль консоли
pub mod consl {
    //Пауза консоли
    pub fn pause() {
        println!("Нажмите, чтобы продолжить... ");

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    }
}

pub mod password {
    use crate::{crypto::aes256::check_password, iomod::input::{input_line, read_u8}};
    use rand::Rng;

    //Массив символов
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";

    //Функция меню
    pub fn menu() -> u8 {
        println!("Выберите действие:\n(1) Сгенирировать пароль\n(2) Сохранить пароль\n(3) Показать пароль\n(4) Сменить логин\n(5) Сменить пароль\n(6) Удалить пароль\n(0) Выход");
        let action: u8 = read_u8();
        if action == 1{
            println!("(1) Сгенирировать пароль\n");
        } else if action == 2 {
            println!("(2) Сохранить пароль\n");
        } else if action == 3{
            println!("(3) Показать пароль");
        } else if action == 4 {
            println!("(4) Сменить логин\n");
        } else if action == 5 {
            println!("(5) Сменить пароль\n");
        } else if action == 6 {
            println!("(6) Удалить пароль\n");
        }
 
        action
    }

    //Генерация пароля
    pub fn generate(length: i8) -> String {
        let mut rng = rand::thread_rng();

        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        password
    }

    // Ввод и проверка пароля 
    pub fn input_password() -> bool {
        let mut pass = String::new();
        println!("Введите пароль:");
        //pass = input_line();
        while pass != "0" {
            pass = input_line();
            if check_password(&pass) == false {
                println!("Неверный пароль! Попробуйте ещё раз, или введите '0' для выхода");
                println!("Введите пароль:");
                //pass = input_line();
            } else {
                break;
            }
        }

        if pass == "0"{
            false
        } else {
            true
        }
    }
}
