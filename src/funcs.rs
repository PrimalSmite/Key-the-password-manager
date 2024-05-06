//Модуль консоли
pub mod consl {
    //Очистка консоли
    pub fn clean() {
        let term = console::Term::stdout();
        term.clear_screen().expect("Не удалось очистить консоль");
    }
}

//Модуль ввода
pub mod input {
    use std::{io, usize};

    //Функция создает переменную из типа String в тип u8
    pub fn transform(string: &String) -> u8 {
        let number: u8 = string.trim().parse().expect(
            "Не удалось распознать число. Пожалуйста, убедитесь, что вы ввели число от 0 до 255",
        );

        number
    }

    //Функция вводит данные с клавиатуры и записывает в переменню типа String
    fn stdinput(mut string: &mut String) -> String {
        //Ввод значения с клавитуры в переменную String
        io::stdin()
            .read_line(&mut string)
            .expect("Не удалось ввести значение");
        let result = String::new();

        result
    }

    //Функция вводит данные с клавиатуры и записывает их в тип u8
    pub fn int_input(mut string: &mut String) -> u8 {
        *string = stdinput(&mut string);
        let number: u8 = transform(&string);

        //Возвращает number:u8
        number
    }

    pub fn usize_transform(string: &String) -> usize {
        let number: usize = string.trim().parse().expect(
            "Не удалось распознать число. Пожалуйста, убедитесь, что вы ввели число от 0 до 255",
        );

        number
    }

    //Функция вводит данные с клавиатуры и записывает их в тип u8
    pub fn usize_input(mut string: &mut String) -> usize {
        *string = stdinput(&mut string);
        let number: usize = usize_transform(&string);

        //Возвращает number:u8
        number
    }
}

pub mod password {
    use crate::funcs::input::int_input;
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    //Функция меню
    pub fn menu() -> u8 {
        println!("Выберите действие:\n(1) Сгенирировать пароль\n(2) Сохранить пароль\n(3) Показать пароль\n(4) Сменить логин\n(5) Сменить пароль\n(6) Удалить пароль\n(0) Выход");

        let mut str_act = String::new();
        let action: u8;

        action = int_input(&mut str_act);

        action
    }

    //Генерация пароля
    pub fn generate(length: usize) -> String {
        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        password
    }
}
